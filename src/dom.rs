use std::{hash::Hash, ops::Deref};

use pb_arena::{sync::Arena, ArenaId};

use crate::{ecs::{
    component::{ComponentRef, Components}, 
    systems::tree::{Tree, TreeExplorer}
}, style::Styles};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind {
    Element,
    Text,
}

/// An interface to explore a document.
pub trait TDocumentObjectModelExplorer: TreeExplorer {
    type Element;
    type Text;

    type ElementRef<'a>: Deref<Target = Self::Element> where Self: 'a;
    type TextRef<'a>: Deref<Target = Self::Text> where Self: 'a;

    fn kind(&self, node: &Self::NodeId) -> NodeKind;

    fn borrow_element(&self, node: &Self::NodeId) -> Option<Self::ElementRef<'_>>;
    fn borrow_text(&self, node: &Self::NodeId) -> Option<Self::TextRef<'_>>;
}

pub type NodeId = ArenaId;
pub struct Element;
pub struct Text;

#[derive(Clone)]
pub struct DocumentObjectModel {
    nodes: Arena<NodeKind>,
    tree: Tree<NodeId>,
    elements: Components<NodeId, Element>,
    texts: Components<NodeId, Text>,
}

impl Deref for DocumentObjectModel {
    type Target = Tree<NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl TDocumentObjectModelExplorer for DocumentObjectModel {
    type Element = Element;
    type Text = Text;

    type ElementRef<'a> = ComponentRef<'a, Element>;
    type TextRef<'a> = ComponentRef<'a, Text>;

    fn kind(&self, node: &Self::NodeId) -> NodeKind {
        *self.nodes.borrow(node).unwrap()
    }

    fn borrow_element(&self, node: &Self::NodeId) -> Option<Self::ElementRef<'_>> {
        self.elements.borrow(node)
    }

    fn borrow_text(&self, node: &Self::NodeId) -> Option<Self::TextRef<'_>> {
        self.texts.borrow(node)
    }

}


/// DOM Handler
pub struct DomHandler<'a, Dom>
where Dom: TDocumentObjectModelExplorer
{
    pub dom: &'a Dom,
    pub(crate) styles:  Styles<Dom::NodeId>,
}

impl<'a, Dom> DomHandler<'a, Dom> 
where Dom: TDocumentObjectModelExplorer
{
    
}

impl<'a, Dom> Clone for DomHandler<'a, Dom> 
where Dom: TDocumentObjectModelExplorer
{
    fn clone(&self) -> Self {
        Self { dom: self.dom.clone(), styles: self.styles.clone() }
    }
}