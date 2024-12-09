use std::borrow::BorrowMut;

use pb_arena::{Arena, ArenaId};
use stylo::properties_and_values::value::ComputedValue;

use crate::{component::Components, font::SizedFont};

pub type BoxNode = ArenaId;

pub enum BoxNodeKind {
    Box,
    TextSequence
}

#[derive(Default)]
pub struct BoxEdge {
    children: Vec<BoxNode>,
    parent: Option<BoxNode> 
}

pub struct TextSequence {
    text: String,
    font: SizedFont
}

pub struct BoxTree {
    pub root: Option<BoxNode>,
    pub nodes: Arena<BoxNodeKind>,
    pub boxes: Components<BoxNode, Box<i32>>,
    pub edges: Components<BoxNode, BoxEdge>,
    pub computed_values: Components<BoxNode, ComputedValue>,
    pub text_sequences: Components<BoxNode, TextSequence>
}

impl BoxTree {
    // Insert a text sequence in the box tree
    pub fn insert_text_sequence(&mut self, text: &str, font: SizedFont, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = self.nodes.alloc(BoxNodeKind::TextSequence);
        self.text_sequences.bind(node, TextSequence{text: text.to_owned(), font});
        self.edges.bind_default(node);
        maybe_parent.inspect(|parent| self.attach_child(parent, &node));
        node
    }

    // Attach the child 
    pub fn attach_child(&self, parent: &BoxNode, child: &BoxNode) {
        self.edges.borrow_mut(parent).unwrap().children.push(*child);
        self.edges.borrow_mut(child).unwrap().parent = Some(*parent);
    }

    /// Insert a box in the box tree
    pub fn insert_box(&mut self, computed_value: ComputedValue, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = self.nodes.alloc(BoxNodeKind::Box);
        self.computed_values.bind(node, computed_value);
        self.boxes.bind_default(node);
        self.edges.bind_default(node);
        maybe_parent.inspect(|parent| self.attach_child(parent, &node));
        node
    }
}