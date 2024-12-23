use std::{borrow::{Borrow, BorrowMut}, hash::Hash, ops::Deref};

use pb_atomic_linked_list::AtomicQueue;

use crate::ecs::component::Components;

pub trait TreeExplorer {
    type NodeId: Copy;
    type ChildIter<'a>: Iterator<Item = Self::NodeId> + 'a
    where
        Self: 'a;

    fn root(&self) -> Option<Self::NodeId>;
    fn parent<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId>;
    fn is_leaf<N: Borrow<Self::NodeId>>(&self, node: N) -> bool;

    fn previous_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId>;
    fn next_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId>;
    fn last_sibling<N: Borrow<Self::NodeId>>(&self, head_sibling: N) -> Option<Self::NodeId>;

    fn first_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId>;
    fn last_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId>;
    fn iter_children<N: Borrow<Self::NodeId>>(&self, parent: N) -> Self::ChildIter<'_>;
}

pub struct AscendingTreeWalker<'a, Tree: TreeExplorer> {
    queue: AtomicQueue<Tree::NodeId>,
    tree: &'a Tree,
}

impl<Tree: TreeExplorer> Iterator for AscendingTreeWalker<'_, Tree> {
    type Item = Tree::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.dequeue() {
            self.tree
                .parent(&node)
                .into_iter()
                .for_each(|child| self.queue.enqueue(child));
            
            return Some(node);
        }

        None
    }
}


/// Breadth-first tree walking
pub struct TreeWalker<'a, Tree: TreeExplorer> {
    queue: AtomicQueue<Tree::NodeId>,
    tree: &'a Tree,
}

pub fn walk_ascendants<'tree, Tree: TreeExplorer>(tree: &'tree Tree, from: &Tree::NodeId) -> AscendingTreeWalker<'tree, Tree> {
    let mut queue = AtomicQueue::new();
    queue.enqueue(*from);
    AscendingTreeWalker { queue, tree }
}
/// Breadth-first tree walking
pub fn walk<Tree: TreeExplorer>(tree: &Tree) -> TreeWalker<'_, Tree> {
    let mut queue = AtomicQueue::new();
    tree.root().iter().for_each(|node| queue.enqueue(*node));
    TreeWalker { queue, tree }
}

pub fn walk_from<'a, Tree: TreeExplorer>(
    tree: &'a Tree,
    node: &Tree::NodeId,
) -> TreeWalker<'a, Tree> {
    let mut queue = AtomicQueue::new();
    queue.enqueue(*node);
    TreeWalker { queue, tree }
}

impl<Tree: TreeExplorer> Iterator for TreeWalker<'_, Tree> {
    type Item = Tree::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.dequeue() {
            self.tree
                .iter_children(&node)
                .for_each(|child| self.queue.enqueue(child));
            return Some(node);
        }

        None
    }
}

pub trait TreeMutator: TreeExplorer {
    /// Split the children of the node based on the predicate
    fn split_children<F: Fn(&Self::NodeId) -> bool>(
        &mut self,
        parent: &Self::NodeId,
        predicate: F,
        mode: SplitMode,
    ) -> Option<Split<Self::NodeId>>;

    fn attach_children(
        &mut self,
        parent: &Self::NodeId,
        children: impl Iterator<Item = Self::NodeId>,
    );
    
    /// Attach a child
    fn attach_child(&mut self, parent: &Self::NodeId, child: Self::NodeId);

    /// Remove a child
    fn remove_child(&mut self, child: Self::NodeId);

    /// Interpose a child between the parent and the parent's children
    fn interpose_child(&mut self, parent: &Self::NodeId, new_child: Self::NodeId);

    /// Push a new sibling
    fn push_sibling(&mut self, node: &Self::NodeId, new_sibling: Self::NodeId);

    /// Pop the next sibling the siblings list
    fn pop_sibling(&mut self, node: &Self::NodeId) -> Option<Self::NodeId>;

    /// Push a new parent 
    fn push_parent(&mut self, node: &Self::NodeId, parent: Self::NodeId);
}

pub struct TreeEdges<EntityId> {
    parent: Option<EntityId>,
    sibling: Option<EntityId>,
    child: Option<EntityId>,
}

impl<EntityId> Default for TreeEdges<EntityId> {
    fn default() -> Self {
        Self {
            parent: None,
            sibling: None,
            child: None,
        }
    }
}

pub enum ChildIter<'a, Tree: TreeExplorer> {
    Empty,
    SiblingIter(SiblingIter<'a, Tree>),
}

impl<Tree: TreeExplorer> Default for ChildIter<'_, Tree> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<Tree: TreeExplorer> Iterator for ChildIter<'_, Tree> {
    type Item = Tree::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChildIter::Empty => None,
            ChildIter::SiblingIter(sibling_iter) => sibling_iter.next(),
        }
    }
}

pub struct SiblingIter<'a, Tree: TreeExplorer> {
    tree: &'a Tree,
    current: Tree::NodeId,
}

impl<'a, Tree: TreeExplorer> SiblingIter<'a, Tree> {
    pub fn new(tree: &'a Tree, head_sibling: Tree::NodeId) -> Self {
        Self {
            tree,
            current: head_sibling,
        }
    }
}

impl<Tree: TreeExplorer> Iterator for SiblingIter<'_, Tree> {
    type Item = Tree::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree
            .next_sibling(&self.current)
            .inspect(|next_sibling| self.current = *next_sibling)
    }
}

#[derive(Clone, Copy)]
pub struct Split<EntityId: Copy> {
    pub left: EntityId,
    pub right: EntityId,
}

pub enum SplitMode {
    Before,
    After,
}

#[derive(Clone)]
pub struct Tree<NodeId: Hash + Copy + Eq + 'static> {
    root: Option<NodeId>,
    edges: Components<NodeId, TreeEdges<NodeId>>,
}

impl<NodeId: Hash + Copy + Eq + 'static> Tree<NodeId> {
    pub fn new() -> Self {
        Self {
            root: None,
            edges: Components::new(100),
        }
    }

    pub fn bind_edges(&mut self, node: &NodeId) {
        self.edges.bind_default(node);
    }
}

impl<NodeId: Hash + Copy + Eq + 'static> Default for Tree<NodeId> {
    fn default() -> Self {
        Self::new()
    }
}

impl<NodeId: Hash + Copy + Eq + 'static> TreeExplorer for Tree<NodeId> {
    type NodeId = NodeId;
    type ChildIter<'a>
        = ChildIter<'a, Self>
    where
        NodeId: 'a;

    fn root(&self) -> Option<Self::NodeId> {
        self.root
    }

    fn parent<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.edges.borrow(node.borrow()).and_then(move |edges| edges.parent)
    }

    fn is_leaf<N: Borrow<Self::NodeId>>(&self, node: N) -> bool {
        self.edges
            .borrow(node.borrow())
            .map(|edge| edge.child.is_none())
            .unwrap_or_else(|| true)
    }

    fn previous_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        let node_id = *node.borrow();
        if let Some(parent) = self.parent(&node_id) {
            for sibling in self.iter_children(&parent) {
                if let Some(edges) = self.edges.borrow(&sibling) {
                    if edges.sibling == Some(node.borrow().clone()) {
                        return Some(sibling);
                    }
                }
            }
        }
        None
    }

    fn next_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.edges.borrow(node.borrow()).and_then(|edge| edge.sibling)
    }

    fn last_sibling<N: Borrow<Self::NodeId>>(&self, from: N) -> Option<Self::NodeId> {
        self.iter_siblings(from.borrow()).last()
    }

    fn first_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.edges.borrow(parent.borrow()).and_then(|edge| edge.child)
    }

    fn last_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.iter_children(parent.borrow()).last()
    }

    fn iter_children<N: Borrow<Self::NodeId>>(&self, parent: N) -> Self::ChildIter<'_> {
        self.first_child(parent)
            .map(|head| self.iter_siblings(&head))
            .map(ChildIter::SiblingIter)
            .unwrap_or_default()
    }
}

impl<NodeId: Hash + Copy + Eq> TreeMutator for Tree<NodeId> {
    fn attach_children(
        &mut self,
        parent: &Self::NodeId,
        children: impl Iterator<Item = Self::NodeId>,
    ) {
        children.for_each(|child| self.attach_child(parent, child));
    }

    fn attach_child(&mut self, parent: &Self::NodeId, child: Self::NodeId) {
        if let Some(tail_sibling) = self.last_child(parent) {
            self.push_sibling(&tail_sibling, child);
        } else if let Some(mut edges) = self.edges.borrow_mut(parent) {
            edges.child = Some(child)
        }

        if let Some(mut edges) = self.edges.borrow_mut(&child) {
            edges.parent = Some(*parent);
        }
    }

    fn push_sibling(&mut self, node: &Self::NodeId, new_sibling: Self::NodeId) {
        let mut maybe_old_sibling: Option<Self::NodeId> = None;

        if let Some(mut edges) = self.edges.borrow_mut(node) {
            maybe_old_sibling = edges.sibling;
            edges.sibling = Some(new_sibling);

            self.edges.borrow_mut(&new_sibling).unwrap().parent = edges.parent;
        }

        if let Some(old_sibling) = maybe_old_sibling {
            self.push_sibling(&new_sibling, old_sibling);
        }
    }

    fn pop_sibling(&mut self, node: &Self::NodeId) -> Option<Self::NodeId> {
        if let Some(mut edges) = self.edges.borrow_mut(node) {
            let sibling = edges.sibling;
            edges.sibling = None;
            return sibling;
        }

        None
    }

    fn split_children<F: Fn(&Self::NodeId) -> bool>(
        &mut self,
        parent: &Self::NodeId,
        predicate: F,
        mode: SplitMode,
    ) -> Option<Split<Self::NodeId>> {
        self.iter_children(parent)
            .find(predicate)
            .and_then(|split_at| match mode {
                SplitMode::Before => self.previous_sibling(&split_at),
                SplitMode::After => Some(split_at),
            })
            .and_then(|left| self.pop_sibling(&left).map(|right| Split { left, right }))
    }
    
    fn remove_child(&mut self, child: Self::NodeId) {
        if let Some(parent) = self.parent(child) {
            // The child we want to remove is the head of the siblings ll
            if let Some(head) = self.first_child(child) {
                if head == child {
                    self.edges.borrow_mut(&parent).unwrap().child = self.next_sibling(head)
                }
            }
            // The child is in a sibling ll
            if let Some(previous) = self.previous_sibling(child) {
                self.pop_sibling(&previous);
            }
        }

    }
    
    fn push_parent(&mut self, node: &Self::NodeId, parent: Self::NodeId) {
        if let Some(previous) = self.previous_sibling(node) {
            self.push_sibling(&previous, parent);
        }

        self.remove_child(*node);
        self.attach_child(&parent, *node);
    }
    
    fn interpose_child(&mut self, parent: &Self::NodeId, new_child: Self::NodeId) {
        // reattach all the children of the parent to the new children
        if let Some(child) = self.first_child(parent) {
            self.iter_siblings(&child)
            .for_each(|child| {
                self.edges.borrow_mut(&child).unwrap().parent = Some(new_child);
            });
        }

        self.attach_child(parent, new_child);
    }
}

impl<NodeId: Hash + Copy + Eq> Tree<NodeId> {
    fn iter_siblings(
        &self,
        from: &<Tree<NodeId> as TreeExplorer>::NodeId,
    ) -> SiblingIter<'_, Tree<NodeId>> {
        SiblingIter::new(self, *from)
    }
}

impl<NodeId: Hash + Copy + Eq + 'static, U> TreeExplorer for U
where
    U: Deref<Target = Tree<NodeId>>,
{
    type NodeId = NodeId;
    type ChildIter<'a>
        = <Tree<NodeId> as TreeExplorer>::ChildIter<'a>
    where
        Self: 'a;

    fn root(&self) -> Option<Self::NodeId> {
        self.deref().root()
    }

    fn parent<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.deref().parent(node)
    }

    fn is_leaf<N: Borrow<Self::NodeId>>(&self, node: N) -> bool {
        self.deref().is_leaf(node)
    }

    fn previous_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.deref().previous_sibling(node)
    }

    fn next_sibling<N: Borrow<Self::NodeId>>(&self, node: N) -> Option<Self::NodeId> {
        self.deref().previous_sibling(node)
    }

    fn last_sibling<N: Borrow<Self::NodeId>>(&self, head_sibling: N) -> Option<Self::NodeId> {
        self.deref().last_sibling(head_sibling)
    }

    fn first_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.deref().first_child(parent)
    }

    fn last_child<N: Borrow<Self::NodeId>>(&self, parent: N) -> Option<Self::NodeId> {
        self.deref().last_child(parent)
    }

    fn iter_children<N: Borrow<Self::NodeId>>(&self, parent: N) -> Self::ChildIter<'_> {
        self.deref().iter_children(parent)
    }
}
