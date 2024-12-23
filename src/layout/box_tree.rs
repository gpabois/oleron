use std::borrow::Borrow;

use pb_arena::{sync::Arena, ArenaId};
use pb_atomic_hash_map::AtomicHashMap;

use crate::{
    ecs::{
        component::Components, 
        systems::tree::{
            Tree, TreeEdges, TreeExplorer, TreeMutator
        }
    }, style::properties::computed
};

use super::{formatting_context::FormattingContextId, text_sequence::TextSequence};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct BoxNode(ArenaId);

#[derive(Clone, Copy)]
pub enum BoxNodeKind {
    Box(u16),
    TextSequence
}

impl BoxNodeKind {
    pub fn is_block_level(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => {
                (flags & LEVEL_MASK) == BLOCK_LEVEL
            },
            _ => false
        }
    }

    pub fn is_inline_level(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => {
                (flags & LEVEL_MASK) == INLINE_LEVEL
            },
            _ => true
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => {
                (flags & ATOMIC_MASK) == ATOMIC
            },
            _ => true
        }
    }

    pub fn is_atomic_inline(&self) -> bool {
        self.is_inline_level() && self.is_atomic()
    }
}

pub type BoxEdges = TreeEdges<BoxNode>;

pub const LEVEL_MASK: u16 = 0b11;
pub const BLOCK_LEVEL: u16 = 0b1;
pub const INLINE_LEVEL: u16 = 0b10;
pub const RUN_IN_LEVEL: u16 = 0b11;
pub const BLOCK_CONTAINER: u16 = 0b100;
pub const ATOMIC_MASK: u16 = 0b1000;
pub const ATOMIC: u16 = 0b1000;

#[derive(Clone)]
pub struct BoxTree<DomNodeId> 
{
    tree: Tree<BoxNode>,
    nodes: Arena<BoxNodeKind>,
    pub dom: AtomicHashMap<BoxNode, DomNodeId>,
    pub boxes: Components<BoxNode, Box<i32>>,
    pub computed_values: Components<BoxNode, computed::Properties>,
    pub text_sequences: Components<BoxNode, TextSequence>,
    pub formatting_contexts: AtomicHashMap<BoxNode, FormattingContextId>
}

impl<DomNodeId> TreeExplorer for BoxTree<DomNodeId>
{
    type NodeId = BoxNode;
    type ChildIter<'a> = <Tree<BoxNode> as TreeExplorer>::ChildIter<'a>
    where
        Self: 'a;

    fn root(&self) -> Option<Self::NodeId> {
        self.tree.root()
    }

    fn parent<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.tree.parent(node)
    }

    fn is_leaf<N: Borrow<Self::NodeId>>(&self, node: N) -> bool {
        self.tree.is_leaf(node)
    }

    fn previous_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
       self.tree.previous_sibling(node)
    }

    fn next_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.tree.next_sibling(node)
    }

    fn last_sibling<N: Borrow<Self::NodeId>>(&self, head_sibling: N) -> Option<Self::NodeId> {
        self.tree.last_sibling(head_sibling)
    }

    fn first_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.tree.first_child(parent)
    }

    fn last_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.tree.last_child(parent)
    }

    fn iter_children<N: Borrow<Self::NodeId>>(&self, parent: N) -> Self::ChildIter<'_> {
        self.tree.iter_children(parent)
    }
}

impl<DomNodeId> TreeMutator for BoxTree<DomNodeId>
{
    fn split_children<F: Fn(&Self::NodeId) -> bool>(
        &mut self,
        parent: &Self::NodeId,
        predicate: F,
        mode: crate::ecs::systems::tree::SplitMode,
    ) -> Option<crate::ecs::systems::tree::Split<Self::NodeId>> {
        self.tree.split_children(parent, predicate, mode)
    }

    fn attach_children(
        &mut self,
        parent: &Self::NodeId,
        children: impl Iterator<Item = Self::NodeId>,
    ) {
        self.tree.attach_children(parent, children);
    }

    fn attach_child(&mut self, parent: &Self::NodeId, child: Self::NodeId) {
        self.tree.attach_child(parent, child);
    }

    fn push_sibling(&mut self, node: &Self::NodeId, new_sibling: Self::NodeId) {
        self.tree.push_sibling(node, new_sibling);
    }

    fn pop_sibling(&mut self, node: &Self::NodeId) -> Option<Self::NodeId> {
        self.tree.pop_sibling(node)
    }
    
    fn remove_child(&mut self, child: Self::NodeId) {
        self.tree.remove_child(child);
    }
    
    fn push_parent(&mut self, node: &Self::NodeId, parent: Self::NodeId) {
        self.tree.push_parent(node, parent);
    }
}

impl<DomNodeId> BoxTree<DomNodeId> 
{
    // Insert a text sequence in the box tree
    pub fn insert_text_sequence(&mut self, text: &str, props: computed::Properties, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = BoxNode(self.nodes.alloc(BoxNodeKind::TextSequence));
        self.text_sequences.bind(&node, TextSequence::from(text));
        self.computed_values.bind(&node, props);
        self.tree.bind_edges(&node);
        maybe_parent.inspect(|parent| self.tree.attach_child(parent, node));
        node
    }

    /// Insert a box in the box tree
    pub fn insert_box(&mut self, flags: u16, props: computed::Properties, maybe_parent: Option<BoxNode>) -> BoxNode {
        let node = BoxNode(self.nodes.alloc(BoxNodeKind::Box(flags))); 
        self.computed_values.bind(&node, props);
        self.boxes.bind_default(&node);
        self.tree.bind_edges(&node);
        maybe_parent.inspect(|parent| self.tree.attach_child(parent, node));
        node
    }

    /// Bind a formatting context to the box node
    pub fn bind_formatting_context<BN: Borrow<BoxNode>>(&mut self, box_node: BN, fc: &FormattingContextId) {
        self.formatting_contexts.insert(box_node.borrow().clone(), *fc)

    }

    pub fn kind<BN: Borrow<BoxNode>>(&self, box_node: BN) -> BoxNodeKind {
        *self.nodes.borrow(&box_node.borrow().0).unwrap()
    }

    pub fn has_inline_level_boxes<BN: Borrow<BoxNode>>(&self, box_node: BN) -> bool {
        self
            .iter_children(box_node)
            .map(|child| self.kind(child))
            .any(|kind| kind.is_block_level())
    }
}