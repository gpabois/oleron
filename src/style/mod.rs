use std::hash::Hash;

use display::DisplayOutside;
use pb_arena::ArenaId;
use properties::{computed, initial, used};

use crate::{
    dom::TDocumentObjectModelExplorer,
    ecs::{component::Components, systems::tree::walk},
};

pub mod parser;
pub mod border;
pub mod display;
pub mod margin;
pub mod order;
pub mod padding;
pub mod properties;
pub mod values;
pub mod visibility;


#[derive(Clone, Copy, Hash)]
pub struct ComputedStyleId(ArenaId);

/// Style system which holds all style applied to any document node.
#[derive(Default, Clone)]
pub struct Styles<NodeId: Hash + Copy> {
    pub initial:    Components<NodeId, initial::Properties>,
    pub computed:   Components<NodeId, computed::Properties>,
    pub used:       Components<NodeId, used::Properties>,
}

impl<NodeId: Hash + Copy> Styles<NodeId> {
    pub fn new(bucket_size: usize, cache_size: usize) -> Self {
        Self {
            initial:    Components::new(bucket_size, cache_size),
            computed:   Components::new(bucket_size, cache_size),
            used:       Components::new(bucket_size, cache_size)
        }
    }

    /// Creates a new style holder which shares the same style pool
    /// as another one.
    pub fn new_shared<OtherNodeId: Hash + Copy>(other: &Styles<OtherNodeId>) -> Self {
        Self {
            initial: Components::new_shared(&other.initial),
            computed: Components::new_shared(&other.computed),
            used: Components::new_shared(&other.used)
        }
    }
}

pub fn style<Dom>(dom: &Dom, style: &mut Styles<Dom::NodeId>)
where
    Dom: TDocumentObjectModelExplorer + Sync,
    Dom::NodeId: Hash + Copy + Eq,
{
    for node in walk(dom) {
        if node == dom.root().unwrap() {
            blockify(dom, &node, style);
        }
    }
}

/// ```spec
/// Some layout effects require blockification or inlinification of the box type, which sets the box’s computed outer display type to block or inline (respectively). (This has no effect on display types that generate no box at all, such as none or contents.)
/// Additionally:
/// - If a block box (block flow) is inlinified, its inner display type is set to flow-root so that it remains a block container.
/// - If an inline box (inline flow) is inlinified, it recursively inlinifies all of its in-flow children, so that no block-level descendants break up the inline formatting context in which it participates.
/// - For legacy reasons, if an inline block box (inline flow-root) is blockified, it becomes a block box (losing its flow-root nature). For consistency, a run-in flow-root box also blockifies to a block box.
/// If a layout-internal box is blockified, its inner display type converts to flow so that it becomes a block container. Inlinification has no effect on layout-internal boxes. (However, placement in such an inline context will typically cause them to be wrapped in an appropriately-typed anonymous inline-level box.)
/// ```
pub fn inlinify<Dom>(dom: &Dom, node: &Dom::NodeId, style: &Styles<Dom::NodeId>)
where
    Dom: TDocumentObjectModelExplorer + Sync,
    Dom::NodeId: Hash + Copy + Eq,
{
    let display = &mut style.computed.borrow_mut(node).unwrap().display;

    if display.r#box().is_some() {
        return;
    }

    if display.is_block_box() {
        display.set_inner(display::DisplayInside::FlowRoot);
    }

    if display.is_inline_box() {
        dom.iter_children(node).for_each(|child| {
            inlinify(dom, &child, style);
        });
    }

    display.set_outer(DisplayOutside::Inline);
}

/// ```spec
/// Some layout effects require blockification or inlinification of the box type, which sets the box’s computed outer display type to block or inline (respectively). (This has no effect on display types that generate no box at all, such as none or contents.)
/// Additionally:
/// - If a block box (block flow) is inlinified, its inner display type is set to flow-root so that it remains a block container.
/// - If an inline box (inline flow) is inlinified, it recursively inlinifies all of its in-flow children, so that no block-level descendants break up the inline formatting context in which it participates.
/// - For legacy reasons, if an inline block box (inline flow-root) is blockified, it becomes a block box (losing its flow-root nature). For consistency, a run-in flow-root box also blockifies to a block box.
/// If a layout-internal box is blockified, its inner display type converts to flow so that it becomes a block container. Inlinification has no effect on layout-internal boxes. (However, placement in such an inline context will typically cause them to be wrapped in an appropriately-typed anonymous inline-level box.)
/// ```
pub fn blockify<Dom>(_dom: &Dom, node: &Dom::NodeId, style: &Styles<Dom::NodeId>)
where
    Dom: TDocumentObjectModelExplorer + Sync,
    Dom::NodeId: Hash + Copy + Eq,
{
    let display = &mut style.computed.borrow_mut(node).unwrap().display;

    if display.r#box().is_some() {
        return;
    }

    if display.internal().is_some() {
        display.set_inner(display::DisplayInside::Flow);
    }

    display.set_outer(DisplayOutside::Block);
}

