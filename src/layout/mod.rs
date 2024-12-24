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

    
}

/// Generate the box node from the DOM node's children
fn generate_box_children_subtrees<Dom>(ctx: &mut RenderingContext<'_, Dom>, dom_node: &Dom::NodeId, parent: BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{   

}

/// Establishes a new block formatting context
fn establishes_new_bfc<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) -> BoxNode
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    ctx.boxes.formatting_contexts.establish_new_formatting_context(box_node, FormattingContext::new_block());
    *box_node
}

/// Establishes a new inline formatting context (Inline-formatting context)
/// 
/// Returns the box which is the root container of the FC
fn establish_new_inline_formatting_context<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) -> BoxNode
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    ctx
        .boxes
        .formatting_contexts
        .establish_new_formatting_context(
            box_node, 
            FormattingContext::new_inline()
        );

    // if the box is a block container
    // creates a root inline element.
    if ctx.boxes.kind(box_node).is_block_container() {
        let root_inline_box = ctx.boxes.insert_box(
            BoxFlags::root_inline_box(), 
            box_node, 
            None
        );

        ctx.boxes.interpose_child(box_node, root_inline_box);

        root_inline_box
    } else {
        *box_node
    }
}

/// If the box is a block container
/// and has only inline-level elements
/// 
/// Then it must establish a new inline formatting context
pub fn check_if_a_new_inline_formatting_context_must_be_established<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    if ctx.boxes.kind(box_node).is_block_container() && ctx.boxes.has_only_inline_level_boxes(box_node) {
        establish_new_inline_formatting_context(ctx, box_node);
    }
}

pub fn check_if_anonymous_box_is_required<Dom>(ctx: &mut RenderingContext<'_, Dom>, box_node: &BoxNode) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    let maybe_parent = ctx.boxes.parent(box_node);

    let requires_anonymous_block_box = maybe_parent
        .map(|parent| 
            ctx.boxes.kind(parent).is_block_container() 
            && ctx.boxes.has_inline_level_boxes(parent)
        )
        .unwrap_or_default();


    if requires_anonymous_block_box {
        let anonymous = ctx
            .boxes
            .insert_box(
                BoxFlags::block_level(), 
                box_node, 
                None
            );
            
        ctx.boxes.push_parent(box_node, anonymous);
    }
}
