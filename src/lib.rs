use std::hash::Hash;

use dom::TDocumentObjectModelExplorer;
use layout::{box_tree::BoxTree, formatting_context::FormattingContexts};
use style::Style;

pub mod dom;
pub mod ecs;
pub mod font;
pub mod style;
pub mod layout;

pub struct RenderingContext<'a, Dom> 
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    pub (crate) dom: &'a Dom,
    pub (crate) style: Style<Dom::NodeId>,
    pub (crate) box_tree: BoxTree<Dom::NodeId>,
}

impl<'a, Dom> Clone for RenderingContext<'a, Dom> 
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    fn clone(&self) -> Self {
        Self { 
            dom: self.dom, 
            style: self.style.clone(), 
            box_tree: self.box_tree.clone(),
        }
    }
}