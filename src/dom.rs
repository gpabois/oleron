#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind {
    Element,
    Text,
}

pub trait TDocumentObjectModel {
    type NodeId;
    type Element;
    type Text;

    type ElementRef<'a>: Deref<Target = &'a Self::Element>;
    type TextRef<'a>: Deref<Target = &'a Self::Text>;

    fn kind(&self, node: &NodeId) -> Self::NodeKind;
    fn borrow_element(&self, node: &NodeId) -> Option<Self::ElementRef<'_>>;
    fn borrow_text(&self, node: &NodeId) -> Option<Self::TextRef<'_>>;

    fn parent(&self, node: &NodeId) -> Option<Self::NodeId>;
}
