pub mod formatting_context;
pub mod box_tree;
pub mod text_sequence;

use std::{collections::VecDeque, hash::Hash};

use box_tree::{BoxNode, BoxTree, BLOCK_CONTAINER, BLOCK_LEVEL, INLINE_LEVEL, RUN_IN_LEVEL};

use crate::{dom::TDocumentObjectModelExplorer, ecs::systems::tree::{walk, TreeExplorer, TreeMutator}, style::{display::{DisplayInside, DisplayOutside}, Style}, RenderingContext};

pub struct Layout;


pub fn generate_box_tree<'dom, Dom>(mut ctx: RenderingContext<'dom, Dom>, dom_node: &Dom::NodeId, parent: Option<BoxNode>) 
where Dom: TDocumentObjectModelExplorer + Sync, Dom::NodeId: Hash + Copy + Eq
{
    let style = ctx.style.computed.borrow(&dom_node).unwrap();
        
    if let Some(bx_dsp) = style.display.r#box() {
        match bx_dsp {
            crate::style::display::DisplayBox::Contents => {
                todo!("implements contents")
            },
            crate::style::display::DisplayBox::None => return,
        }
    } 

    if let Some(inner) = style.display.outer() {
        let outer = style.display.inner().unwrap_or(DisplayInside::Flex);

        match outer {
            DisplayInside::Flow => {
                match inner {
                    DisplayOutside::Block => {
                        ctx.box_tree.insert_box(
                            BLOCK_LEVEL,
                            style.clone(),
                            parent
                        );
                    },
                    DisplayOutside::Inline => {
                        ctx.box_tree.insert_box(
                            INLINE_LEVEL, 
                            style.clone(), 
                            parent
                        );
                    },
                    DisplayOutside::RunIn => {
                        ctx.box_tree.insert_box(
                            RUN_IN_LEVEL, 
                            style.clone(), 
                            parent
                        );
                    },
                }
            },

            DisplayInside::FlowRoot => {
                let bn = ctx.box_tree.insert_box(
                    BLOCK_CONTAINER, 
                    style.clone(), 
                    parent
                );
                let fc = ctx.formatting_contexts.new_block_formatting_context();
                ctx.box_tree.bind_formatting_context(bn, &fc);

                ctx.dom.iter_children(dom_node)
                .for_each(|child_dom_node| {
                    generate_box_tree(
                        ctx.clone(), 
                        &child_dom_node, 
                        Some(bn)
                    );
                });


                
            },
            
            DisplayInside::Table => todo!(),
            DisplayInside::Flex => todo!(),
            DisplayInside::Grid => todo!(),
            DisplayInside::Ruby => todo!(),
        }
    }
}

pub fn push_anonymous_block_level_box<DomNodeId>(box_tree: &mut BoxTree<DomNodeId>, box_node: &BoxNode) {
    let props = box_tree.computed_values.borrow(box_node).unwrap().clone();
    let anonymous_box = box_tree.insert_box(BLOCK_LEVEL, props, None);
    box_tree.push_parent(box_node, anonymous_box);
}

pub fn check_if_anonymous_block_boxes_are_required<DomNodeId>(
    box_tree: &mut BoxTree<DomNodeId>, 
    box_node: &BoxNode
) {
    if box_tree.has_inline_level_boxes(box_node) {
        let inline_boxes = box_tree
            .iter_children(box_node)
            .filter(|node| box_tree.kind(node).is_inline_level())
            .collect::<Vec<_>>();

        for inline_box in inline_boxes {
            push_anonymous_block_level_box(box_tree, &inline_box);
        }
    }
}