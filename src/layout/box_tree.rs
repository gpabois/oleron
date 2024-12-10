use pb_arena::{Arena, ArenaId};
use stylo::properties_and_values::value::ComputedValue;

use crate::{r#box::Box, ecs::{component::{ComponentMutRef, ComponentRef, Components}, systems::{boxes::BoxSystem, tree::{TreeEdges, TreeSystem}}}, font::SizedFont};

pub type BoxNode = ArenaId;

#[derive(Clone, Copy)]
pub enum BoxNodeKind {
    Box,
    InlineBox,
    AtomicInline,
    TextSequence
}


pub type BoxEdges = TreeEdges<BoxNode>;

pub struct TextSequence {
    text: String,
    font: SizedFont
}

impl TextSequence {
    pub fn split_by_line_breaks(&self) -> impl Iterator<Item=TextSequence> + '_ {
        self.text.split(|ch| ch == '\n').map(
            |text| TextSequence { 
                text: text.to_owned(), 
                font: self.font.clone() 
            })
    }
}

pub struct BoxTree {
    pub root: Option<BoxNode>,
    pub nodes: Arena<BoxNodeKind>,
    pub boxes: Components<BoxNode, Box<i32>>,
    pub edges: Components<BoxNode, BoxEdges>,
    pub computed_values: Components<BoxNode, ComputedValue>,
    pub text_sequences: Components<BoxNode, TextSequence>
}

impl<'a> BoxSystem<'a, i32> for BoxTree {
    type EntityId = BoxNode;
    type BoxRef = ComponentRef<'a, Box<i32>>;
    type BoxMutRef = ComponentMutRef<'a, Box<i32>>;

    fn borrow_box(&'a self, entity: &Self::EntityId) -> Option<Self::BoxRef> {
        self.boxes.borrow(entity)
    }

    fn borrow_mut_box(&'a self, entity: &Self::EntityId) -> Option<Self::BoxMutRef> {
        self.boxes.borrow_mut(entity)
    }

    fn bind_box(&mut self, entity: &Self::EntityId, bx: Box<i32>) {
        self.boxes.bind(entity, bx)
    }
}

impl TreeSystem for BoxTree {
    type EntityId = BoxNode;
    type EdgesRef<'a> = ComponentRef<'a, BoxEdges>;
    type EdgesMutRef<'a> = ComponentMutRef<'a, BoxEdges>;

    fn borrow_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesRef<'_>> {
        self.edges.borrow(node)
    }

    fn borrow_mut_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesMutRef<'_>> {
        self.edges.borrow_mut(node)
    }

    fn bind_edges(&mut self, node: &Self::EntityId, edges: TreeEdges<Self::EntityId>) {
        self.edges.bind(node, edges)
    }

    fn root(&self) -> Option<Self::EntityId> {
        self.root
    }

    fn copy_node_attributes(&mut self, node: &Self::EntityId) -> Self::EntityId {
        let kind = *self.nodes.borrow(*node).unwrap();
        self.nodes.alloc(kind)
    }
}

impl BoxTree {
    // Insert a text sequence in the box tree
    pub fn insert_text_sequence(&mut self, text: &str, font: SizedFont, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = self.nodes.alloc(BoxNodeKind::TextSequence);
        self.text_sequences.bind(&node, TextSequence{text: text.to_owned(), font});
        self.bind_default_edges(&node);
        maybe_parent.inspect(|parent| self.attach_child(parent, node));
        node
    }

    /// Insert a box in the box tree
    pub fn insert_box(&mut self, computed_value: ComputedValue, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = self.nodes.alloc(BoxNodeKind::Box);
        self.computed_values.bind(&node, computed_value);
        self.boxes.bind_default(&node);
        self.bind_default_edges(&node);
        maybe_parent.inspect(|parent| self.attach_child(parent, node));
        node
    }
}