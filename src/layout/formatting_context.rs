use std::hash::Hash;

use pb_arena::{sync::Arena, ArenaId};
use pb_atomic_hash_map::AtomicHashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FormattingContextId(ArenaId);

#[derive(Clone)]
pub struct FormattingContexts<NodeId>
where NodeId: Hash + Copy + Eq
{
    /// A pointer to the formatting contexts
    formatting_contexts: Arena<FormattingContext>,
    
    /// The FC witch the node established for its content
    pub (crate) establishes: AtomicHashMap<NodeId, FormattingContextId>,

}

impl<NodeId> FormattingContexts<NodeId> 
where NodeId: Hash + Copy + Eq
{
    pub fn new() -> Self {
        Self {
            formatting_contexts: Arena::new(100, 100),
            establishes: AtomicHashMap::new(100),
        }
    }

    pub fn establish_new_formatting_context(&mut self, node: &NodeId, fc: FormattingContext) -> FormattingContextId {
        let fci = FormattingContextId(self.formatting_contexts.alloc(fc));
        self.establishes.insert(*node, fci);
        fci
    }
}

pub enum FormattingContext {
    Inline(InlineFormattingContext),
    Block(BlockFormattingContext)
}

impl FormattingContext {
    pub fn new_inline() -> Self {
        Self::Inline(InlineFormattingContext)
    }

    pub fn new_block() -> Self {
        Self::Block(BlockFormattingContext)
    }

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