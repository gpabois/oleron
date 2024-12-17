
use pb_arena::{Arena, ArenaId, ArenaRef, ArenaRefMut};

use crate::{
    r#box::Box, 
    ecs::{
        component::{ComponentMutRef, ComponentRef, Components}, 
        systems::{boxes::BoxSystem, tree::{TreeEdges, TreeSystem}}
    }
};

use super::{box_tree::{BoxNode, TextSequence}, fragmentation::IsFragmentable, Block, Inline, Lay, Layout};

pub type Fragment = ArenaId;


pub struct FragmentTree {
    pub root: Option<Fragment>,
    // The fragments
    pub fragments: Arena<FragmentKind>,
    // The box which fragment came from
    pub sources: Components<Fragment, BoxNode>,
    // The edges of the fragment
    pub edges: Components<Fragment, FragmentEdges>,
    // The boxes of the fragment
    pub boxes: Components<Fragment, Box<i32>>,
    // Text sequences
    pub text_sequences: Components<Fragment, TextSequence>,
    // Line boxes data
    pub line_boxes: Components<Fragment, LineBox>
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

    /// Checks if the fragment is a break.
    pub fn is_break(&self, fragment: &Fragment) -> bool {
        self.is(fragment, FragmentKind::Break)
    }

    /// Checks if the fragment contains a break.
    pub fn is_breakable(&self, fragment: &Fragment) -> bool {
        self
            .iter_children(fragment)
            .any(|child| self.is_break(&child))
    }

    /// Checks if the fragment is an inline-level content.
    pub fn is_inline_level_content(&self, fragment: &Fragment) -> bool {
        self.kind(fragment).is_inline_level_content()
    }

    /// Returns true if the fragment contains only inline-level content.
    pub fn contains_only_inline_level_content(&self, fragment: &Fragment) -> bool {
        self.iter_children(fragment).all(|child| self.is_inline_level_content(&child))
    }

    pub fn insert_break(&mut self) -> Fragment {
        let fragment = self.fragments.alloc(FragmentKind::Break);
        self.bind_default_edges(&fragment);
        fragment
    }

    /// Insert a text sequence
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

    // Clone a fragment but do not keep its edges.
    pub fn clone_fragment(&mut self, src: &Fragment) -> Fragment {
        let kind = self.kind(src);
        let clone: ArenaId = self.fragments.alloc(kind);

        self.boxes.clone_component(src, &clone);
        self.line_boxes.clone_component(src, &clone);
        self.sources.clone_component(src, &clone);
        self.text_sequences.clone_component(src, &clone);

        clone
    }

}

impl IsFragmentable<Inline> for FragmentTree {
    fn is_fragmentable(&self, fragment: &Fragment) -> bool {
        if self.is(fragment, FragmentKind::AtomicInline) {
            return false;
        }

        if !self.has_children(fragment) {
            return false;
        }

        return true;
    }
}


impl Layout<Block> for FragmentTree {
    type Element = Fragment;

    fn layout(&mut self, element: &Self::Element) {
        
    }
}

impl Layout<Inline> for FragmentTree {
    type Element = Fragment;

    fn layout(&mut self, element: &Self::Element) {
        
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
            FragmentKind::Break => {
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

    fn clone_node(&mut self, node: &Self::EntityId) -> Self::EntityId {
        self.clone_fragment(node)
    }
}

pub type FragmentEdges = TreeEdges<Fragment>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FragmentKind {
    // A text sequence
    TextSequence,
    // A break in a fragmentation flow
    Break,
    // A line box
    LineBox,
    // An inline box that cannot be fragmented
    AtomicInline,
    // A box participating in an inline-level layout
    InlineBox,
    // A block container
    BlockContainer,
    //
    BlockBox
}

impl FragmentKind {
    pub fn is_inline_level_content(&self) -> bool {
        matches!(self, Self::InlineBox | Self::TextSequence)
    }
}

#[derive(Clone)]
pub struct LineBox {
    logical_width: i32
}






