use pb_arena::{sync::Arena, ArenaId};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FormattingContextId(ArenaId);

#[derive(Clone)]
/// The formatting context tree
pub struct FormattingContexts {
    arena: Arena<FormattingContext>,
}

impl FormattingContexts {
    pub fn new_inline_formatting_context(&mut self) -> FormattingContextId {
        FormattingContextId(self.arena.alloc(FormattingContext::Inline(InlineFormattingContext)))
    }

    pub fn new_block_formatting_context(&mut self) -> FormattingContextId {
        FormattingContextId(self.arena.alloc(FormattingContext::Block(BlockFormattingContext)))
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