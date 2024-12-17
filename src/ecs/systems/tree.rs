use std::ops::{Deref, DerefMut};

pub struct TreeEdges<EntityId> {
    parent: Option<EntityId>,
    sibling: Option<EntityId>,
    child: Option<EntityId>
} 

impl<EntityId> Default for TreeEdges<EntityId> {
    fn default() -> Self {
        Self {
            parent: None,
            sibling: None,
            child: None
        }
    }
}

pub enum ChildIter<'a, Tree: TreeSystem>  {
    Empty,
    SiblingIter(SiblingIter<'a, Tree>)
}

impl<'a, Tree: TreeSystem> Default for ChildIter<'a, Tree> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'a, Tree: TreeSystem> Iterator for ChildIter<'a, Tree> {
    type Item = Tree::EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChildIter::Empty => None,
            ChildIter::SiblingIter(sibling_iter) => sibling_iter.next(),
        }
    }
}


pub struct SiblingIter<'a, Tree: TreeSystem> {
    tree: &'a Tree,
    current: Tree::EntityId
}

impl<'a, Tree: TreeSystem> SiblingIter<'a, Tree> {
    pub fn new(tree: &'a Tree, head_sibling: Tree::EntityId) -> Self {
        Self {
            tree,
            current: head_sibling
        }
    }
}

impl<'a, Tree: TreeSystem> Iterator for SiblingIter<'a, Tree> {
    type Item = Tree::EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree
        .borrow_edges(&self.current)
        .map(|edges| edges.sibling)
        .flatten()
        .inspect(|next_sibling| self.current = *next_sibling)
    }
}

#[derive(Clone, Copy)]
pub struct Split<EntityId: Copy> {
    left: EntityId,
    right: EntityId
}

// A trait to represents a tree
pub trait TreeSystem: Sized {
    type EntityId: Copy + PartialEq;
    type EdgesRef<'a> : Deref<Target=TreeEdges<Self::EntityId>> + 'a where Self: 'a;
    type EdgesMutRef<'a>: DerefMut + Deref<Target=TreeEdges<Self::EntityId>> + 'a where Self: 'a;

    // Returns the current tree root
    fn root(&self) -> Option<Self::EntityId>;

    // Borrow the entity's edges component
    fn borrow_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesRef<'_>>;
    
    // Mut borrow the entity's edges component.
    fn borrow_mut_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesMutRef<'_>>;

    // Attach an edges component to the entity (required for all tree-related operations)
    fn bind_edges(&mut self, node: &Self::EntityId, edges: TreeEdges<Self::EntityId>);

    // Attach a default edges component to the entity
    fn bind_default_edges(&mut self, node: &Self::EntityId) {
        self.bind_edges(node, TreeEdges::default());
    }

    // Copy the node attributes
    fn clone_node(&mut self, node: &Self::EntityId) -> Self::EntityId;

    // Check if the node has children
    fn has_children(&self, node: &Self::EntityId) -> bool {
        return !self.is_leaf(node)
    }

    // Check if the node has no children
    fn is_leaf(&self, node: &Self::EntityId) -> bool {
        self
        .borrow_edges(node)
        .map(|edges| edges.sibling.is_none())
        .unwrap_or_else(|| true)
    }
    
    // Push a new sibling in the LL
    fn push_sibling(&self, node: &Self::EntityId, new_sibling: Self::EntityId) {
        let mut maybe_old_sibling: Option<Self::EntityId> = None;

        if let Some(mut edges) = self.borrow_mut_edges(&node) {
            maybe_old_sibling = edges.sibling;
            edges.sibling = Some(new_sibling);

            self.borrow_mut_edges(&new_sibling).unwrap().parent = edges.parent;
        }

        if let Some(old_sibling) = maybe_old_sibling {
            self.push_sibling(&new_sibling, old_sibling);
        }
    }

    /// Remove the sibling of the node
    fn pop_sibling(&self, node: &Self::EntityId) -> Option<Self::EntityId> {
        if let Some(mut edges) = self.borrow_mut_edges(node) {
            let sibling = edges.sibling;
            edges.sibling = None;
            return sibling
        }

        None
    }

    // Split the children [0;at[ & [at; n]
    fn split_at(&self, at: &Self::EntityId)  -> Option<Split<Self::EntityId>> {
        if let Some(parent) = self.parent(at) {
            self.split_children(
                &parent, 
                |child| *at == *child
            )
        } else {
            None
        }
    }

    // Split the chilren at the node which predicate result is true.
    fn split_children(&self, parent: &Self::EntityId, predicate: impl Fn(&Self::EntityId) -> bool) -> Option<Split<Self::EntityId>> {
        self.iter_children(parent)
        .find(predicate)
        .and_then(|split_at| {
            self.before( &split_at)
        })
        .and_then(|left| {
            self
            .pop_sibling(&left)
            .map(|right| Split {left, right})
        })
    }

    /// Get the parent of the node
    fn parent(&self, node: &Self::EntityId) -> Option<Self::EntityId> {
        let edges = self.borrow_edges(node);
        
        edges.and_then(move |edges| edges.parent)
    }

    /// Get the node before the child
    fn before(&self, node: &Self::EntityId) -> Option<Self::EntityId> {
        if let Some(parent) = self.parent(node) {
            for sibling in self.iter_children(&parent) {
                if let Some(edges) = self.borrow_edges(&sibling) {
                    if edges.sibling == Some(*node) {
                        return Some(sibling)
                    }
                }
            }
        }


        None
    }

    fn after(&self, node: &Self::EntityId) -> Option<Self::EntityId> {
        self.borrow_edges(node).and_then(|edges| edges.sibling)
    }

    // Returns the last child in the LL
    fn last_child(&self, parent: &Self::EntityId) -> Option<Self::EntityId> {
        self.borrow_edges(&parent)
        .and_then(|edges| edges.child)
        .and_then(|child |self.last_sibling(&child))
    }

    // Returns the last sibling in the LL
    fn last_sibling(&self, head_sibling: &Self::EntityId) -> Option<Self::EntityId> {
        let iter = SiblingIter::new(self, *head_sibling);
        iter.last()
    }

    // Iterate over children
    fn iter_children(&self, parent: &Self::EntityId) -> ChildIter<'_, Self> {
        self.borrow_edges(parent)
        .map(|edges| edges.child.map(|child| ChildIter::SiblingIter(self.iter_siblings(&child))))
        .flatten()
        .unwrap_or_default()
    }

    // Iterate over siblings
    fn iter_siblings(&self, head_sibling: &Self::EntityId) -> SiblingIter<'_, Self> {
        SiblingIter::new(self, *head_sibling)
    }

    // Attach children
    fn attach_children(&self, parent: &Self::EntityId, children: impl Iterator<Item=Self::EntityId>) {
        children.for_each(|child| self.attach_child(parent, child));
    }

    // Attach a child 
    fn attach_child(&self, parent: &Self::EntityId, child: Self::EntityId) {
        if let Some(tail_sibling) = self.last_child(parent) {
            self.push_sibling(&tail_sibling, child);
        } else if let Some(mut edges)  = self.borrow_mut_edges(&parent) {
            edges.child = Some(child)
        }

        if let Some(mut edges) = self.borrow_mut_edges(&child) {
            edges.parent = Some(*parent);
        }
    }
}