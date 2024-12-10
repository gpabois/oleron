use std::collections::VecDeque;

use pb_arena::{Arena, ArenaId, ArenaRef, ArenaRefMut};

use crate::{
    r#box::Box, 
    ecs::{
        component::{ComponentMutRef, ComponentRef, Components}, 
        systems::{boxes::BoxSystem, tree::{Split, TreeEdges, TreeSystem}}
    }
};

use super::{box_tree::{BoxNode, TextSequence}, Inline, Lay};

pub type Fragment = ArenaId;

pub struct FragmentTree {
    root: Option<Fragment>,
    // The fragments
    fragments: Arena<FragmentKind>,
    // The box which fragment came from
    sources: Components<Fragment, BoxNode>,
    // The edges of the fragment
    edges: Components<Fragment, FragmentEdges>,
    // The boxes of the fragment
    boxes: Components<Fragment, Box<i32>>,
    // Text sequences
    text_sequences: Components<Fragment, TextSequence>,
    // Line boxes data
    line_boxes: Components<Fragment, LineBox>
}

impl FragmentTree {
    pub fn kind(&self, node: &Fragment) -> FragmentKind {
        *self.fragments.borrow(*node).unwrap()
    }
    // Checks the kind of the fragment
    pub fn is(&self, node: &Fragment, kind: FragmentKind) -> bool {
        self.fragments
        .borrow(*node)
        .map(|fragment| *fragment == kind)
        .unwrap_or_default()
    }

    // Checks if the fragment is fragmentable
    pub fn is_fragmentable(&self, node: &Fragment) -> bool {
        if self.is(node, FragmentKind::AtomicInline) {
            return false;
        }

        if !self.has_children(node) {
            return false;
        }

        return true;
    }

    pub fn insert_line_break(&mut self) -> Fragment {
        self.fragments.alloc(FragmentKind::LineBreak)
    }

    pub fn insert_text_sequence(&mut self, seq: TextSequence) -> Fragment {
        let fragment = self.fragments.alloc(FragmentKind::TextSequence);
        self.text_sequences.bind(&fragment, seq);
        fragment
    }

    // Insert an atomic inline block
    // Atomic inlines cannot be fragmented.
    pub fn insert_atomic_inline(&mut self, r#box: Box<i32>, source: BoxNode, maybe_parent: Option<Fragment>) -> Fragment {
        let fragment = self.fragments.alloc(FragmentKind::AtomicInline);
        self.bind_box(&fragment, r#box);
        self.sources.bind(&fragment, source);
        maybe_parent.map(|parent| self.attach_child(&parent, fragment));
        fragment
    }

    // Insert a common box fragment
    pub fn insert_inline_box(&mut self, bx: Box<i32>, source: BoxNode, maybe_parent: Option<Fragment>) -> Fragment {
        let fragment = self.fragments.alloc(FragmentKind::InlineBox);
        self.bind_box(&fragment, bx);
        self.bind_default_edges(&fragment);
        self.sources.bind(&fragment, source);
        maybe_parent.map(|parent| self.attach_child(&parent, fragment));
        fragment
    }

    // Insert a line box fragment
    pub fn insert_line_box(&mut self, logical_width: i32, source: BoxNode, maybe_parent: Option<Fragment>) -> Fragment {
        let fragment = self.fragments.alloc(FragmentKind::LineBox);
        self.line_boxes.bind(&fragment, LineBox {logical_width});
        self.bind_default_edges(&fragment);
        self.bind_default_box(&fragment);
        self.sources.bind(&fragment, source);
        maybe_parent.map(|parent| self.attach_child(&parent, fragment));
        fragment
    }

}

impl FragmentTree {
        /// Compute, or recompute the box of a fragment.
    /// 
    /// Depending on the kind of fragment, the function can be called
    /// recursively.
    pub fn compute_box(&self, from: &Fragment) -> Box<i32> {
        match self.kind(from) {
            FragmentKind::TextSequence => {
                let line_box = self
                .iter_children(from)
                .map(|child| self.compute_box(&child))
                .reduce(Lay::<Inline>::lay)
                .unwrap_or_default();

                let mut r#box = self.borrow_mut_box(from).unwrap();
                *r#box = line_box.clone();
                line_box
            },
            // No dimensions for a line break
            FragmentKind::LineBreak => {
                let zero = Box::<i32>::default();
                let mut r#box = self.borrow_mut_box(from).unwrap();
                *r#box = zero.clone();
                zero
            },
            FragmentKind::LineBox => {
                let line_box = self
                .iter_children(from)
                .map(|child| self.compute_box(&child))
                .reduce(Lay::<Inline>::lay)
                .unwrap_or_default();

                let mut r#box = self.borrow_mut_box(from).unwrap();
                *r#box = line_box.clone();
                line_box
            },
            FragmentKind::AtomicInline => {
                return self.borrow_box(from).unwrap().clone()
            },
            FragmentKind::InlineBox => {
                let line_box = self
                .iter_children(from)
                .map(|child| self.compute_box(&child))
                .reduce(Lay::<Inline>::lay)
                .unwrap_or_default();

                let mut r#box = self.borrow_mut_box(from).unwrap();
                *r#box = line_box.clone();
                line_box
            },
        }
    }
}

impl FragmentTree {
    /// Fragment a text sequence on a line break.
    ///
    /// # Arguments
    /// - `text_sequence` - Must be a text sequence fragment
    pub fn fragment_on_line_break(&mut self, text_sequence: &Fragment) {
        if let FragmentKind::TextSequence = self.kind(text_sequence) {
            let mut seq = self.text_sequences.borrow_mut(text_sequence).unwrap();
            let mut frag_seq = seq.split_by_line_breaks().collect::<VecDeque<_>>();

            if let Some(head) = frag_seq.pop_front() {
                *seq = head;
            }

            drop(seq);
    
            let mut previous = *text_sequence;

            while let Some(sub_seq) = frag_seq.pop_front() {
                let brk = self.insert_line_break();
                self.push_sibling(&previous, brk);
                
                let seq_frag = self.insert_text_sequence(sub_seq);
                self.push_sibling(&brk, seq_frag);
                previous = seq_frag;
    
            }
        }
    }

    /// Fragment any boxes which width exceeds the max width.
    pub fn fragment_with_max_width(&mut self, max_width: i32, node: &Fragment) -> Option<Split<Fragment>> {
        let r#box = self.borrow_box(&node).unwrap();
        if r#box.content.width <= max_width {
            return None;
        }

        if !self.is_fragmentable(&node) {
            return None;
        }

        
    }
}

impl<'a> BoxSystem<'a, i32> for FragmentTree {
    type EntityId = Fragment;
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

impl TreeSystem for FragmentTree {
    type EntityId = Fragment;
    type EdgesRef<'a> = ArenaRef<'a, FragmentEdges>;
    type EdgesMutRef<'a> = ArenaRefMut<'a, FragmentEdges>;

    fn root(&self) -> Option<Self::EntityId> {
        self.root.clone()
    }

    fn borrow_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesRef<'_>> {
        self.edges.borrow(node)
    }

    fn borrow_mut_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesMutRef<'_>> {
        self.edges.borrow_mut(node)
    }

    fn bind_edges(&mut self, node: &Self::EntityId, edges: crate::ecs::systems::tree::TreeEdges<Self::EntityId>) {
        self.edges.bind(&node, edges)
    }

    fn copy_node_attributes(&mut self, node: &Self::EntityId) -> Self::EntityId {
        let kind = *self.fragments.borrow(*node).unwrap();
        self.fragments.alloc(kind)
    }
}

pub type FragmentEdges = TreeEdges<Fragment>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FragmentKind {
    // A text sequence
    TextSequence,
    // A line break
    LineBreak,
    // A line box
    LineBox,
    // An inline box that cannot be fragmented
    AtomicInline,
    // A box participating in an inline-level 
    InlineBox
}

pub struct LineBox {
    logical_width: i32
}






