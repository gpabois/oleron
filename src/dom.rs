use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind {
    Element,
    Text,
}

pub trait TDocumentObjectModel {
    type NodeId;
    type Element;
    type Text;

    type ElementRef<'a>: Deref<Target = &'a Self::Element> where Self: 'a;
    type TextRef<'a>: Deref<Target = &'a Self::Text> where Self: 'a;

    fn kind(&self, node: &Self::NodeId) -> NodeKind;
    fn borrow_element(&self, node: &Self::NodeId) -> Option<Self::ElementRef<'_>>;
    fn borrow_text(&self, node: &Self::NodeId) -> Option<Self::TextRef<'_>>;

    fn parent(&self, node: &Self::NodeId) -> Option<Self::NodeId>;
}
