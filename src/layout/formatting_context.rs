use pb_arena::{Arena, ArenaId, ArenaRef, ArenaRefMut};

use crate::ecs::{component::Components, systems::tree::{TreeEdges, TreeSystem}};

pub type FormattingContextId = ArenaId;

/// The formatting context tree
pub struct FormattingContextTree {
    root: Option<FormattingContextId>,
    arena: Arena<FormattingContext>,
    edges: Components<FormattingContextId, TreeEdges<FormattingContextId>>
}

impl TreeSystem for FormattingContextTree {
    type EntityId = FormattingContextId;
    type EdgesRef<'a>  = ArenaRef<'a, TreeEdges<FormattingContextId>>;
    type EdgesMutRef<'a> = ArenaRefMut<'a, TreeEdges<FormattingContextId>>;

    fn root(&self) -> Option<Self::EntityId> {
        self.root
    }

    fn borrow_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesRef<'_>> {
        self.edges.borrow(node)
    }

    fn borrow_mut_edges(&self, node: &Self::EntityId) -> Option<Self::EdgesMutRef<'_>> {
        self.edges.borrow_mut(node)
    }

    fn bind_edges(&mut self, node: &Self::EntityId, edges: TreeEdges<Self::EntityId>) {
        self.edges.bind(node, edges);
    }

    fn clone_node(&mut self, node: &Self::EntityId) -> Self::EntityId {
        todo!()
    }
}

pub enum FormattingContext {
    Inline(InlineFormattingContext),
    Block(BlockFormattingContext)
}

impl FormattingContext {
    pub fn kind(&self) -> FormattingContextKind {
        match self {
            FormattingContext::Inline(_) => FormattingContextKind::InlineFormattingContext,
            FormattingContext::Block(_) => FormattingContextKind::BlockFormattingContext,
        }
    }
}

pub enum FormattingContextKind {
    InlineFormattingContext,
    BlockFormattingContext
}

pub struct InlineFormattingContext;
pub struct BlockFormattingContext;