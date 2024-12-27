use super::lexer::{Lexer, TokenKind};

pub enum ParserError {}
pub type ParserResult<T> = Result<T, ParserError>;

pub enum Rule {
    At(AtRule),
    Qualified(QualifiedRule)
}

impl From<AtRule> for Rule {
    fn from(value: AtRule) -> Self {
        Rule::At(value)
    }
}

impl From<QualifiedRule> for Rule {
    fn from(value: QualifiedRule) -> Self {
        Rule::Qualified(value)
    }
}

pub struct AtRule;
pub struct QualifiedRule;

pub fn consume_at_rule(lexer: &mut Lexer<'_>) -> ParseResult<AtRule> {

}

pub fn consume_qualified_rule(lexer: &mut Lexer<'_>) -> ParserResult<QualifiedRule> {
    
}

pub fn consume_list_of_rules(lexer: &mut Lexer<'_>) {
    let mut rules = Vec::<Rule>::default();

    while let Some(Ok(tok)) = lexer.peek(0) {
        if tok.kind == TokenKind::Whitespace {
            lexer.consume(1);
        }

        if matches!(tok.kind, TokenKind::At) {
            consume_at_rule(lexer)
                .into_iter()
                .map(Rule::from)
                .for_each(|rule| rules.push(rule));
        }

        consume_qualified_rule(lexer)
            .into_iter()
            .map(Rule::from)
            .for_each(|rule| rules.push(rule));
    }
}