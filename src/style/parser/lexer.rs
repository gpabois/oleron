use std::{ops::Range, str::CharIndices};

pub enum TokenKind {
    Comment,
    Newline,
    Whitespace,
    Url,
    BadUrl,
    Ident,
    At
}

pub struct Token<'src> {
    value: &'src str,
    kind: TokenKind
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Span<T> {
    loc: Range<usize>,
    data: T
}

#[derive(Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
    loc: Range<usize>
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { 
            input, 
            chars: input.char_indices(),
            loc: Range::default()
        }
    }
}

impl<'input> Lexer<'input> {
    pub fn next_char(&mut self) -> Option<char> {
        self.chars.next().map(|(_, ch)| ch)
    }
    pub fn peek_char(&self, nth: usize) -> Option<char> {
        self.chars.clone()
            .nth(nth)
            .map(|(i, ch)| ch)
    }

    pub fn consume(&mut self, nth: usize) {
        for _ in 0..nth {
            self.chars.next();
        }
    }

    pub fn peek_str(&self, nth: usize) -> &'input str {
        let start = self.current().unwrap().0;
        &self.input[start..start+nth]
    }

    pub fn current(&self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    pub fn set_current_as_start(&mut self) {
        self.current().into_iter().for_each(|(i, _)| {
            self.loc = i..self.loc.end
        });
    }

    pub fn set_current_as_end(&mut self) {
        self.current().into_iter().for_each(|(i, _)| {
            self.loc = self.loc.start..i
        });
    }

    pub fn as_str(&mut self) -> Span<&'input str> {
        Span {
            loc: self.loc.clone(),
            data: &self.input[self.loc.clone()]
        }
    }

    pub fn is_whitespace(ch: char) -> bool {
        ch == ' ' || ch == '\t'
    }

    pub fn consume_url_token(&mut self) -> Option<Span<Token<'input>>> {
        if self.peek_str(3) == "url(" {
            self.set_current_as_start();
            let start = self.loc.start;
            
            self.consume(4);
            self.consume_whitespaces();
            self.set_current_as_start();

            while let Some(ch) = self.peek_char(1) {
                match ch {
                    // consume whitespaces
                    ch if Self::is_whitespace(ch) => continue,
                    // bad-url
                    '"' | '\'' | '(' => {
                        self.consume(1);
                        
                        while let Some(ch) = self.next_char()  {
                            self.consume(1);
                            if ch == ')' {
                                break;
                            }
                        }

                        self.set_current_as_end();
                        let value = &self.input[self.loc.clone()];
                        return Some(Span {
                            loc: self.loc.clone(),
                            data: Token {
                                value,
                                kind: TokenKind::BadUrl
                            }
                        })

                    },
                    // escape
                    '\\' => {
                        self.consume(2);
                        self.set_current_as_end();
                    },
                    ')' => {
                        let loc = self.loc.clone();
                        let value = &self.input[loc];

                        self.consume(1);
                        self.loc.start = start;
                        self.set_current_as_end();

                        return Some(Span {
                            loc: self.loc.clone(),
                            data: Token {
                                value,
                                kind: TokenKind::Url
                            }
                        })
                    },
                    _ => {
                        self.consume(1);
                        self.set_current_as_end();
                    }
                }
            }
        }

        None
    }

    pub fn consume_whitespaces(&mut self) -> Option<Span<Token<'input>>> {
        match self.peek_char(1) {
            Some(ch) if Self::is_whitespace(ch) => {
                self.set_current_as_start();
                
                while self.peek_char(1).map(Self::is_whitespace).unwrap_or_default() {
                    self.consume(1);
                }
                
                self.set_current_as_end();
                Some(Span {
                    loc: self.loc.clone(),
                    data: Token {
                        value: self.as_str().data,
                        kind: TokenKind::Whitespace
                    }
                })
            },
            _ => None
        }
    }

}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<Token<'input>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek().and_then(|(i, ch)| match ch {
            ' ' | '\t' => self.consume_whitespaces(),
            '\n' | '\r' => self.consume_newline(),
            'u' => {
                if self.peek_str(3) == "url(" {
                    self.consume_url_token()
                } else {
                    self.consume_ident_token()
                }
            },
            '-' => {

            },
            _ => panic!("")
        });
    }
}
