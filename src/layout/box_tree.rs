use std::borrow::Borrow;

use pb_arena::{sync::Arena, ArenaId};
use pb_atomic_hash_map::AtomicHashMap;

use crate::{
    ecs::{
        component::Components, 
        systems::tree::{
            walk_ascendants, Tree, TreeEdges, TreeExplorer, TreeMutator
        }
    }, style::properties::computed
};

use super::{formatting_context::{FormattingContextId, FormattingContexts}, text_sequence::TextSequence};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct BoxNode(ArenaId);

#[derive(Clone, Copy)]
pub enum BoxNodeKind {
    Box(BoxFlags),
    TextSequence
}

impl BoxNodeKind {
    pub fn is_block_container(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => flags.is_block_container(),
            _ => false       
        }
    }

    pub fn is_block_level(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => flags.is_block_level(),
            _ => false
        }
    }

    pub fn is_inline_level(&self) -> bool {
        match self {
            BoxNodeKind::Box(flags) => flags.is_inline_level(),
            _ => true
        }
    }

    pub fn is_atomic_inline(&self) -> bool {
        match self {
            BoxNodeKind::Box(box_flags) => box_flags.is_atomic_inline_level(),
            BoxNodeKind::TextSequence => false,
        }
    }
}

pub type BoxEdges = TreeEdges<BoxNode>;


#[derive(Clone, Copy, Default)]
pub struct BoxFlags(u16);

impl std::ops::BitAnd<u16> for BoxFlags {
    type Output = u16;

    fn bitand(self, rhs: u16) -> Self::Output {
        self.0 & rhs
    }
}

impl std::ops::BitOr<u16> for BoxFlags {
    type Output = u16;

    fn bitor(self, rhs: u16) -> Self::Output {
        self.0 | rhs
    }
}

impl BoxFlags {
    pub const LEVEL_MASK: u16 = 0b11;
    pub const BLOCK_LEVEL: u16 = 0b1;
    pub const INLINE_LEVEL: u16 = 0b10;
    pub const RUN_IN_LEVEL: u16 = 0b11;
    
    pub const CONTAINER_MASK: u16 = 0b100;
    pub const CONTAINER: u16 = 0b100;

    pub const ATOMIC_MASK: u16 = 0b1000;
    pub const ATOMIC: u16 = 0b1000;
    
    pub const ROOT: u16 = 0b10000;

    pub fn is_inline_level(&self) -> bool {
        (*self & Self::LEVEL_MASK) == Self::INLINE_LEVEL
    }

    pub fn is_block_level(&self) -> bool {
        (*self & Self::LEVEL_MASK) == Self::BLOCK_LEVEL
    }

    pub fn is_block_container(&self) -> bool {
        return self.is_container() && self.is_block_level()
    }

    pub fn is_atomic_inline_level(&self) -> bool {
        return self.is_inline_level() && self.is_atomic()
    }
    
    fn is_container(&self) -> bool {
        return (*self & Self::CONTAINER_MASK) == Self::CONTAINER
    }

    fn is_atomic(&self) -> bool {
        return (*self & Self::ATOMIC_MASK) == Self::ATOMIC
    }

    pub const fn run_in_level() -> Self {
        Self(Self::RUN_IN_LEVEL)
    }

    pub const fn block_level() -> Self {
        Self(Self::BLOCK_LEVEL)
    }

    pub const fn block_container() -> Self {
        Self(Self::BLOCK_LEVEL | Self::CONTAINER)
    }

    pub const fn inline_level() -> Self {
        Self(Self::INLINE_LEVEL)
    }

    pub const fn root_inline_box() -> Self {
        Self(Self::INLINE_LEVEL | Self::ROOT)
    }
}


#[derive(Clone)]
pub struct BoxTree<DomNodeId> 
{
    tree: Tree<BoxNode>,
    nodes: Arena<BoxNodeKind>,
    pub dom: AtomicHashMap<BoxNode, DomNodeId>,
    pub boxes: Components<BoxNode, Box<i32>>,
    pub computed_values: Components<BoxNode, computed::Properties>,
    pub text_sequences: Components<BoxNode, TextSequence>,
    pub formatting_contexts: FormattingContexts<BoxNode>
}

pub enum ComputedProperties {
    Properties(computed::Properties),
    SameAs(BoxNode)
}

impl From<computed::Properties> for ComputedProperties {
    fn from(value: computed::Properties) -> Self {
        Self::Properties(value)
    }
}

impl From<BoxNode> for ComputedProperties {
    fn from(value: BoxNode) -> Self {
        Self::SameAs(value)
    }
}

impl From<&BoxNode> for ComputedProperties {
    fn from(value: &BoxNode) -> Self {
        Self::SameAs(*value)
    }
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
    
    fn interpose_child(&mut self, parent: &Self::NodeId, new_child: Self::NodeId) {
        self.tree.interpose_child(parent, new_child);
    }
}

impl<DomNodeId> BoxTree<DomNodeId> {
    /// Returns the formatting context in which the nodes is participant.
    pub fn get_formatting_context(&self, node: &BoxNode) -> Option<FormattingContextId> {
        for asc in walk_ascendants(self, node).skip(1) {
            if let Some(fc) = self.formatting_contexts.establishes.borrow(&asc).as_deref().copied() {
                return Some(fc)
            }
        }
        None
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
    pub fn insert_box<Props>(&mut self, flags: BoxFlags, props: Props, maybe_parent: Option<BoxNode>) -> BoxNode 
    where ComputedProperties: From<Props>
    {
        let node = BoxNode(self.nodes.alloc(BoxNodeKind::Box(flags))); 
        
        match ComputedProperties::from(props) {
            ComputedProperties::Properties(props) => {
                self.computed_values.bind(&node, props); 
            },
            ComputedProperties::SameAs(from) => {
                self.computed_values.share_from(&node, &from);
            },
        }

        self.boxes.bind_default(&node);
        self.tree.bind_edges(&node);
        maybe_parent.inspect(|parent| self.tree.attach_child(parent, node));
        node
    }

    pub fn kind<BN: Borrow<BoxNode>>(&self, box_node: BN) -> BoxNodeKind {
        *self.nodes.borrow(&box_node.borrow().0).unwrap()
    }

    pub fn has_only_inline_level_boxes<BN: Borrow<BoxNode>>(&self, box_node: BN) -> bool {
        self
        .iter_children(box_node)
        .map(|child| self.kind(child))
        .all(|kind| kind.is_inline_level())      
    }

    pub fn has_inline_level_boxes<BN: Borrow<BoxNode>>(&self, box_node: BN) -> bool {
        self
            .iter_children(box_node)
            .map(|child| self.kind(child))
            .any(|kind| kind.is_inline_level())
    }
}