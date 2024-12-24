use std::hash::Hash;

use dom::{DomHandler, TDocumentObjectModelExplorer};
use layout::box_tree::BoxTree;
use style::Styles;

pub mod dom;
pub mod ecs;
pub mod font;
pub mod style;
pub mod layout;


pub struct RenderingContextArgs<'a, Dom> 
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    dom: &'a Dom,
    bucket_size: Option<usize>,
    cache_size: Option<usize>,
}

pub struct RenderingContext<'a, Dom> 
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    pub (crate) dom: DomHandler<'a, Dom>,
    pub (crate) boxes: BoxTree<Dom::NodeId>,
}

impl<'a, Dom> RenderingContext<'a, Dom>
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    pub fn new(args: RenderingContextArgs<'a, Dom>) -> Self {
        let styles = Styles::new(
            args.bucket_size.unwrap_or(100),
            args.cache_size.unwrap_or(100)
        );
        
        let dom = DomHandler {dom: args.dom, styles};

        let boxes = BoxTree::new(
            &dom.styles,
            args.bucket_size.unwrap_or(100),
            args.cache_size.unwrap_or(100)
        );

        Self {
            dom,
            boxes
        }
    }
}
impl<'a, Dom> Clone for RenderingContext<'a, Dom> 
where Dom: TDocumentObjectModelExplorer, Dom::NodeId: Hash + Copy + Eq
{
    fn clone(&self) -> Self {
        Self { 
            dom: self.dom.clone(), 
            boxes: self.boxes.clone(),
        }
    }
}