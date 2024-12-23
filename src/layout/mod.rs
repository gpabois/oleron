pub mod formatting_context;
pub mod box_tree;
pub mod text_sequence;

use std::hash::Hash;

use box_tree::{BoxFlags, BoxNode};
use formatting_context::FormattingContext;

use crate::{dom::TDocumentObjectModelExplorer, ecs::systems::tree::{TreeExplorer, TreeMutator}, style::display::{DisplayInside, DisplayOutside}, RenderingContext};


/// ```spec
/// Floats, absolutely positioned elements, block containers (such as inline-blocks, table-cells, and table-captions) that are not block boxes, and block boxes with 'overflow' other than 'visible' (except when that value has been propagated to the viewport) establish new block formatting contexts for their contents.
/// ```
pub fn generate_box_subtree_with_parent<'dom, Dom>(mut ctx: RenderingContext<'dom, Dom>, dom_node: &Dom::NodeId, maybe_parent: Option<BoxNode>) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    let style = ctx.style.computed.borrow(&dom_node).unwrap().clone();
        
    if let Some(bx_dsp) = style.display.r#box() {
        match bx_dsp {
            crate::style::display::DisplayBox::Contents => {
                todo!("implements contents")
            },
            crate::style::display::DisplayBox::None => return,
        }
    } 

    let box_node = if let Some(inner) = style.display.outer() {
        let outer = style.display.inner().unwrap_or(DisplayInside::Flex);

        match outer {
            DisplayInside::Flow => {
                match inner {
                    DisplayOutside::Block => {
                        ctx.box_tree.insert_box(
                            BoxFlags::block_level(),
                            style.clone(),
                            maybe_parent
                        )
                    },
                    DisplayOutside::Inline => {
                        ctx.box_tree.insert_box(
                            BoxFlags::inline_level(), 
                            style.clone(), 
                            maybe_parent
                        )
                    },
                    DisplayOutside::RunIn => {
                        ctx.box_tree.insert_box(
                            BoxFlags::run_in_level(), 
                            style.clone(), 
                            maybe_parent
                        )
                    },
                }
            },

            DisplayInside::FlowRoot => {
                let box_node = ctx.box_tree.insert_box(
                    BoxFlags::block_container(), 
                    style.clone(), 
                    maybe_parent
                );

                establishes_new_bfc(&mut ctx, &box_node)
            },
            
            DisplayInside::Table => todo!(),
            DisplayInside::Flex => todo!(),
            DisplayInside::Grid => todo!(),
            DisplayInside::Ruby => todo!(),
        } 
    }   else {
        todo!()
    };
    
    // Check if we have to push a new anonymous box
    check_if_anonymous_box_is_required(&mut ctx, &box_node);

    // Generate the rest of the box tree
    generate_box_children(&mut ctx, dom_node, box_node);

    // If it's a block container, and has only inline-level elements
    // Then it mush establishes an IFC.
    check_if_a_new_inline_formatting_context_must_be_established(&mut ctx, &box_node);
    
}

/// Generate the box node from the DOM node's children
fn generate_box_children<Dom>(ctx: &mut RenderingContext<'_, Dom>, dom_node: &Dom::NodeId, parent: BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{   
    ctx
    .dom
    .iter_children(dom_node)
    .for_each(|child_dom_node| {
        generate_box_subtree_with_parent(
            ctx.clone(), 
            &child_dom_node, 
            Some(parent)
        );
    });
}

/// Establishes a new block formatting context
fn establishes_new_bfc<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) -> BoxNode
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    *box_node
}

/// Establishes a new inline formatting context (Inline-formatting context)
/// 
/// Returns the box which is the root container of the FC
fn establish_new_inline_formatting_context<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) -> BoxNode
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    ctx
        .box_tree
        .formatting_contexts
        .establish_new_formatting_context(
            box_node, 
            FormattingContext::new_inline()
        );

    // if the box is a block container, creates a root inline element.
    if ctx.box_tree.kind(box_node).is_block_container() {
        let root_inline_box = ctx.box_tree.insert_box(
            BoxFlags::root_inline_box(), 
            box_node, 
            None
        );

        ctx.box_tree.interpose_child(box_node, root_inline_box);

        root_inline_box
    } else {
        *box_node
    }
}

pub fn check_if_a_new_inline_formatting_context_must_be_established<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    if ctx.box_tree.kind(box_node).is_block_container() && ctx.box_tree.has_only_inline_level_boxes(box_node) {
        establish_new_inline_formatting_context(ctx, box_node);
    }
}

pub fn check_if_anonymous_box_is_required<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    let maybe_parent = ctx.box_tree.parent(box_node);

    let requires_anonymous_block_box = maybe_parent
        .map(|parent| 
            ctx.box_tree.kind(parent).is_block_container() 
            && ctx.box_tree.has_inline_level_boxes(parent)
        )
        .unwrap_or_default();


    if requires_anonymous_block_box {
        let anonymous = ctx
            .box_tree
            .insert_box(
                BoxFlags::block_level(), 
                box_node, 
                None
            );
            
        ctx.box_tree.push_parent(box_node, anonymous);
    }
}
