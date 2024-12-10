use std::{collections::VecDeque, ops::Deref};

use crate::ecs::systems::{boxes::BoxSystem, tree::TreeSystem};

use super::fragment_tree::{Fragment, FragmentKind, FragmentTree};


pub trait FragmentStrategy {
    fn fragment(&self, fragment_tree: &mut FragmentTree, node: &Fragment);
}

/// Pre-fragment inline boxes
pub struct MaxWidthFragmentation(pub i32);

impl Deref for MaxWidthFragmentation {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FragmentStrategy for MaxWidthFragmentation {

    fn fragment(&self, fragment_tree: &mut FragmentTree, node: &Fragment) {
        let r#box = fragment_tree.borrow_box(&node).unwrap();
        
        // We don't need to fragment it.
        if r#box.content.width <= self.0 {
            return;
        }

        // We cannot fragment it.
        if !fragment_tree.is_fragmentable(&node) {
            return;
        }

        

    }
}


pub struct LineBreakFragmentation;
impl FragmentStrategy for LineBreakFragmentation {

    /// Fragment a text sequence on a line break.
    ///
    /// # Arguments
    /// - `text_sequence` - Must be a text sequence fragment
    fn fragment(&self, fragment_tree: &mut FragmentTree, node: &Fragment) {
        if let FragmentKind::TextSequence = fragment_tree.kind(node) {
            let mut seq = fragment_tree.text_sequences.borrow_mut(node).unwrap();
            let mut frag_seq = seq.split_by_line_breaks().collect::<VecDeque<_>>();

            if let Some(head) = frag_seq.pop_front() {
                *seq = head;
            }

            drop(seq);
    
            let mut previous = *node;

            while let Some(sub_seq) = frag_seq.pop_front() {
                let brk = fragment_tree.insert_line_break();
                fragment_tree.push_sibling(&previous, brk);
                
                let seq_frag = fragment_tree.insert_text_sequence(sub_seq);
                fragment_tree.push_sibling(&brk, seq_frag);
                previous = seq_frag;
    
            }
        }
    }
}

