use std::{cmp::min, ops::{Deref, Range, RangeInclusive}, str::CharIndices};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'input> {
    Comment,
    Delim,
    Hash(bool),
    Number(Number<'input>),
    Dimension(Dimension<'input>),
    Percentage(Number<'input>),
    CDC,
    CDO,
    Comma,
    Colon,
    Semicolon,
    LeftPar,
    RightPar,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    String,
    Newline,
    Whitespace,
    Function,
    Url,
    BadUrl,
    Ident,
    At,
    EOF
}

pub type SpannedToken<'src> = Span<Token<'src>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub value: &'src str,
    pub kind: TokenKind<'src>
}

impl<'src> Token<'src> {
    pub fn new(value: &'src str, kind: TokenKind<'src>) -> Self {
        Self{value, kind}
    }
}

#[derive(Debug)]
pub enum LexicalError {
    UnexpectedEof
}

pub type LexerResult<T> = Result<T, LexicalError>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Exponent<'input> {
    pub neg: bool,
    pub value: &'input str
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Number<'input> {
    neg: bool,
    integer: Option<&'input str>,
    decimal: Option<&'input str>,
    exponent: Option<Exponent<'input>>
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Dimension<'input> {
    pub number: Number<'input>,
    pub unit: &'input str
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<T> {
    pub loc: RangeInclusive<usize>,
    pub value: T
}

impl<T> Deref for Span<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Span<T> {
    pub fn new(value: T, loc: RangeInclusive<usize>) -> Self {
        Self {value, loc }
    }
}

#[derive(Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { 
            input, 
            chars: input.char_indices(),
        }
    }
}

impl<'input> Lexer<'input> {
    fn peek_indice(&self, nth: usize) -> Option<(usize, char)> {
        self.chars.clone().nth(nth)
    }

    fn peek_char(&self, nth: usize) -> Option<char> {
        self.chars.clone()
            .nth(nth)
            .map(|(_, ch)| ch)
    }

    fn consume_char(&mut self, n: usize) {
        for _ in 0..n {
            self.chars.next();
        }
    }

    fn peek_str(&self, size: usize) -> &'input str {
        if self.current_pos().is_none() {
            return ""
        }

        let start = self.current_pos().unwrap();
        let end = min(start+size-1, self.input.len() - 1);
        &self.input.get(start..=end).unwrap_or_default()
    }
    
    fn current_pos(&self) -> Option<usize> {
        self.peek_indice(0).map(|(pos, _)| pos)
    }

    fn consume_comment(&mut self) -> LexerResult<Span<Token<'input>>>  {
        assert!(self.peek_str(2) == "/*");

        let start = self.current_pos().unwrap();

        while self.peek_str(2) != "*/" {
            self.consume_char(1);
        }

        self.consume_char(2);
        let end = self.current_pos().unwrap();

        Ok(Span {
            loc: start..=end,
            value: Token {
                kind: TokenKind::Comment,
                value: &self.input[start..=end]
            }
        })
    }


    fn consume_whitespaces(&mut self) {
        self.consume_while(Self::is_whitespace);
    }

    fn consume_whitespace_token(&mut self) -> LexerResult<Span<Token<'input>>> {
        let (loc, value) = self.consume_while(Self::is_whitespace).unwrap();
        Ok(Span {
            loc,
            value: Token {
                kind: TokenKind::Whitespace,
                value
            }
        })
    }

    fn consume_string_token(&mut self) -> LexerResult<Span<Token<'input>>> {        
        let expected_delimiter = self.peek_char(0)
            .ok_or_else(|| LexicalError::UnexpectedEof)?;
        
        let start = self.current_pos()
            .ok_or_else(|| LexicalError::UnexpectedEof)?;
        
        self.consume_char(1); // consume delimiter

        let str_start = self.current_pos()
            .ok_or_else(|| LexicalError::UnexpectedEof)?;
        let mut str_end = str_start - 1;

        while self.peek_char(0) != Some(expected_delimiter) {
            // an escape
            if self.current_is_valid_escape() {
                self.consume_char(2);
                str_end += 2;
            } else {
                self.consume_char(1);
                str_end += 1;
            }
        }   

        // consume the other 
        if self.peek_char(0) == Some(expected_delimiter) {
            self.consume_char(1);
        }

        let end = self.current_pos()
            .ok_or_else(|| LexicalError::UnexpectedEof)?;

        let value = &self.input[str_start..=str_end];

        Ok(Span {
            loc: start..=end,
            value: Token {
                kind: TokenKind::String,
                value
            }
        })

    }

    fn consume_url_token(&mut self) -> LexerResult<Span<Token<'input>>> {
        assert!(self.peek_str(4) == "url(");
        let start = self.current_pos().unwrap();
        let mut end = start;
        
        self.consume_char(4);
        self.consume_whitespaces();

        if self.peek_char(0) == Some('"') || self.peek_char(0) == Some('\'') {
            end += 3;
            return Ok(Span {
                loc: start..=end,
                value: Token {
                    kind: TokenKind::Function,
                    value: &"url("
                }
            })
        }

        let mut url_value_start = self.current_pos().unwrap();
        let mut url_value_end = url_value_start;

        while let Some(ch) = self.peek_char(0) {
            match ch {
                // consume whitespaces
                ch if Self::is_whitespace(ch) => {
                    self.consume_whitespaces();
                },
                // bad-url
                '"' | '\'' | '(' => {
                    end = self.current_pos().unwrap();
                    self.consume_char(1);
                    
                    while let Some((pos, ch)) = self.peek_indice(0)  {
                        end = pos;
                        self.consume_char(1);
                             
                        if ch == ')' {
                            break;
                        } else {
                            url_value_end = self.current_pos().unwrap()
                        }
                    }                   
                    let value = &self.input[url_value_start..url_value_end];
                    return Ok(Span {
                        loc: start..=end,
                        value: Token {
                            value,
                            kind: TokenKind::BadUrl
                        }
                    })

                },
                // escape
                '\\' => {
                    self.consume_char(2);
                    url_value_start += 2;
                },
                ')' => {
                    let value = &self.input[url_value_start..=url_value_end];

                    end = self.current_pos().unwrap();
                    self.consume_char(1);
                    
                    return Ok(Span {
                        loc: start..=end,
                        value: Token {
                            value,
                            kind: TokenKind::Url
                        }
                    })
                },
                _ => {
                    url_value_end = self.current_pos().unwrap();
                    self.consume_char(1);
                }
            }
        }

        unreachable!("unexpected eof")
    }

    fn consume_ident_sequence(&mut self) -> (RangeInclusive<usize>, &'input str) {
        let start = self.current_pos().unwrap();
        let mut end = start;

        while let Some(ch) = self.peek_char(0) {
            if Self::is_valid_escape(self.peek_str(2)) {
                self.consume_char(2);
                end += 2;
            } else if Self::is_ident_code_point(ch) {
                self.consume_char(1);
                end += 1;
            } else {
                break;
            }
        }

        (start..=end-1, &self.input[start..=end-1])
    }

    fn consume_hash(&mut self) -> LexerResult<Span<Token<'input>>> {
        assert!(self.peek_char(0) == Some('#'));
        let start = self.current_pos().unwrap();
        
        self.consume_char(1);

        if self.current_would_start_ident_sequence() {
            let (id_loc, value) = self.consume_ident_sequence();

            return Ok(Span {
                loc: start..=*id_loc.end(),
                value: Token {
                    kind: TokenKind::Hash(true),
                    value
                }
            })

        } else {
            Ok(Span {
                loc: start..=start,
                value: Token {
                    kind: TokenKind::Delim,
                    value: &"#"
                }
            })
        }
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> Option<(RangeInclusive<usize>, &'input str)> {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        while self.peek_char(0).map(&predicate).unwrap_or_default() {
            if start.is_none() {
                start = self.current_pos();
            }

            end = self.current_pos();
            self.consume_char(1);
        }
        
        start.map(|start| {
            let loc = start..=(end.unwrap());
            (loc.clone(), &self.input[loc])
        })
    }

    fn consume_digit_sequence(&mut self) -> Option<(RangeInclusive<usize>, &'input str)> {
        self.consume_while(Self::is_digit)
    }

    fn consume_signed_digit_sequence(&mut self) -> Option<(RangeInclusive<usize>, &'input str, bool, &'input str)> {
        let start = self.current_pos()?;

        let neg_sign = self.peek_char(0).map(|ch| ch == '-').inspect(|_| self.consume_char(1)).unwrap_or_default();
        
        let (loc, digits) = self.consume_digit_sequence()?;
        let end = *loc.end();

        Some((start..=end, &self.input[start..=end], neg_sign, digits))

    }

    fn consume_number(&mut self) -> LexerResult<(RangeInclusive<usize>, Number<'input>)> {
        let mut nb = Number::default();

        let start = self.current_pos().unwrap();
        let mut end = start;

        nb.neg = self.peek_char(0).map(|ch| ch == '-').inspect(|_| self.consume_char(1)).unwrap_or_default();

        self.consume_digit_sequence()
        .into_iter()
        .for_each(|(l, integer)| {
            nb.integer = Some(integer);
            end = *l.end();
        });

        if self.peek_char(0) == Some('.') {
            self.consume_char(1);
            let (l, decimal) = self.consume_digit_sequence().unwrap();
            end = *l.end();
            nb.decimal = Some(decimal);
        }

        if self.peek_char(0) == Some('e') || self.peek_char(0) == Some('E') {
            self.consume_char(1);
            let (l, _, neg, value) = self.consume_signed_digit_sequence().unwrap();
            end = *l.end();
            nb.exponent = Some(Exponent {
                neg,
                value
            });
        }

        Ok((start..=end, nb))
    }

    fn consume_numeric_token(&mut self) -> LexerResult<Span<Token<'input>>> {
        let (num_loc, number) = self.consume_number()?;

        let start = *num_loc.start();
        let mut end = *num_loc.end();
        
        if self.current_would_start_ident_sequence() {
            let (dim_loc, unit) = self.consume_ident_sequence();
            end = *dim_loc.end();

            return Ok(Span {
                loc: start..=end,
                value: Token {
                    kind: TokenKind::Dimension(Dimension { number, unit }),
                    value: &self.input[start..=end]
                }
            });
        }

        if self.current_is_percentage_sign() {
            end = self.current_pos().unwrap();
            self.consume_char(1);
            
            return Ok(Span {
                loc: start..=end,
                value: Token {
                    kind: TokenKind::Percentage(number),
                    value: &self.input[start..=end]
                }
            });
        }

        return Ok(Span {
            loc: start..=end,
            value: Token {
                kind: TokenKind::Number(number),
                value: &self.input[start..=end]
            }
        });    

    }

    fn consume_ident_token(&mut self) -> LexerResult<Span<Token<'input>>> {
        // url-token
        if self.peek_str(4) == "url(" {
            return self.consume_url_token();
        }

        let (mut loc, value) = self.consume_ident_sequence();
        // function-token 
        if self.peek_char(0) == Some('(') {
            self.consume_char(1);
            loc = *loc.start()..=*loc.end()+1;
            
            return Ok(Span {
                loc: loc.clone(),
                value: Token {
                    kind: TokenKind::Function,
                    value: &self.input[loc]
                }
            })
        }

        // ident-token
        Ok(Span {
            loc,
            value: Token {
                kind: TokenKind::Ident,
                value
            }
        })
    }

    fn consume_at_token(&mut self) -> LexerResult<Span<Token<'input>>> {
        let start = self.current_pos().unwrap();
        self.consume_char(1);

        if self.current_would_start_ident_sequence() {
            let (loc, value) = self.consume_ident_sequence();
            return Ok(Span{
                loc: start..=*loc.end(),
                value: Token {
                    kind: TokenKind::At,
                    value
                }
            })
        }

        return Ok(Span{
            loc: start..=start,
            value: Token {
                kind: TokenKind::Delim,
                value: "@"
            }
        })
    }
}

impl Lexer<'_> {
    pub fn current_is_ident_code_point(&self) -> bool {
        self.peek_char(0).map(Self::is_ident_code_point).unwrap_or_default()
    }

    pub fn current_is_valid_escape(&self) -> bool {
        Self::is_valid_escape(self.peek_str(2))
    }

    pub fn current_would_start_ident_sequence(&self) -> bool {
        Self::would_start_and_ident_sequence(self.peek_str(3))
    }

    pub fn current_would_start_number(&self) -> bool {
        if self.peek_char(0).map(Self::is_digit).unwrap_or_default() {
            return true;
        }

        if self.peek_char(0).map(|ch| ch == '+' || ch == '-').unwrap_or_default() {
            if self.peek_char(1) == Some('.') && self.peek_char(2).map(Self::is_digit).unwrap_or_default() {
                return true;
            }

            if self.peek_char(1).map(Self::is_digit).unwrap_or_default() {
                return true;
            }
        }

        if self.peek_char(0).map(|ch| ch == '.').unwrap_or_default() {
            return self.peek_char(2).map(Self::is_digit).unwrap_or_default();
        }

        return false;
    }

    pub fn current_is_percentage_sign(&self) -> bool {
        self.peek_char(0) == Some('%')
    }
}

impl Lexer<'_> {
    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    fn is_whitespace(ch: char) -> bool {
        ch == ' ' || ch == '\t' || ch == '\n'
    }

    fn is_valid_escape(txt: &str) -> bool {
        txt.len() == 2 
        && txt.starts_with("\\")
    }

    fn is_ident_code_point(ch: char) -> bool {
        Self::is_ident_start_code_point(ch)
        || ch.is_numeric()
        || ch == '-'
    }

    fn is_ident_start_code_point(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    fn would_start_and_ident_sequence(txt: &str) -> bool {
        if let Some(first) = txt.chars().nth(0) {
            if Self::is_ident_start_code_point(first) {
                return true;
            }

            if first == '\\' {
                return Self::is_valid_escape(txt)
            }

            if first == '-' {
                if let Some(second) = txt.chars().nth(1) {
                    return second == '-'
                        || Self::is_ident_code_point(second)
                        || Self::is_valid_escape(&txt[1..=2]);
                }
            }
        }

        false
    }
}

impl<'input> Lexer<'input> {
    pub fn peek(&self, n: usize) -> Option<LexerResult<SpannedToken<'input>>> {
        self.clone().nth(n)
    }

    pub fn consume(&mut self, size: usize) {
        for _ in 0..size {
            self.next();
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerResult<Span<Token<'input>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek_char(0).map(|ch| match ch {
            '/' if self.peek_str(2) == "/*" => self.consume_comment(),
            '"' | '\'' => self.consume_string_token(),
            ch if Self::is_whitespace(ch) => self.consume_whitespace_token(),
            '#' => self.consume_hash(),
            '('  => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::LeftPar,
                        value: &"("
                    }
                })
            },
            ')' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::RightPar,
                        value: &")"
                    }
                })
            },
            ',' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Comma,
                        value: &","
                    }
                })               
            },
            '+' if self.current_would_start_number() => self.consume_numeric_token(),
            '+' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value: &"+"
                    }
                })               
            },
            '-' if self.current_would_start_number() => self.consume_numeric_token(),
            '-' if self.current_would_start_ident_sequence() => self.consume_ident_token(),
            '-' if self.peek_str(3) == "-->" => {
                let start = self.current_pos().unwrap();
                self.consume_char(3);
                Ok(Span {
                    loc: start..=start+2,
                    value: Token {
                        kind: TokenKind::CDC,
                        value: &"-->"
                    }
                })  
            },
            '-' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value: &"-"
                    }
                })   
            },
            '.' if self.current_would_start_number() => self.consume_numeric_token(),
            '.' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value: &"."
                    }
                })  
            },
            ':' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Colon,
                        value: &":"
                    }
                })  
            },
            ';' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Semicolon,
                        value: &";"
                    }
                })  
            },
            '<' if self.peek_str(4) == "<!--" => {
                let start = self.current_pos().unwrap();
                self.consume_char(4);
                Ok(Span {
                    loc: start..=start+3,
                    value: Token {
                        kind: TokenKind::Semicolon,
                        value: &"<!--"
                    }
                })  
            },
            '<' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value: &"<"
                    }
                })               
            }
            '@' => self.consume_at_token(),
            '[' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::LeftSquareBracket,
                        value: &"["
                    }
                })               
            },
            '\\' if self.current_is_valid_escape() => self.consume_ident_token(),
            '\\' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value: &"\\"
                    }
                })               
            },
            ']' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::RightSquareBracket,
                        value: &"]"
                    }
                })               
            },
            '{' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::LeftCurlyBracket,
                        value: &"{"
                    }
                })               
            },
            '}' => {
                let start = self.current_pos().unwrap();
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::RightCurlyBracket,
                        value: &"}"
                    }
                })               
            },
            ch if Self::is_digit(ch) => self.consume_numeric_token(),
            ch if Self::is_ident_start_code_point(ch) => self.consume_ident_token(),
            _ => {
                let start = self.current_pos().unwrap();
                let value = self.peek_str(1);
                self.consume_char(1);
                Ok(Span {
                    loc: start..=start,
                    value: Token {
                        kind: TokenKind::Delim,
                        value
                    }
                })               
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::style::parser::lexer::Exponent;

    use super::{Lexer, LexerResult, Token, Span, TokenKind, Number, Dimension};

    #[test]
    fn test_whitespace_token() {
        let lexer = Lexer::new("   ");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().ok().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "   ",
                        TokenKind::Whitespace
                    ),
                    0..=2
                )
            ]
        )
    }

    #[test]
    fn test_ident_token() {
        let lexer = Lexer::new("--0123id_token");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "--0123id_token",
                        TokenKind::Ident
                    ),
                    0..=13
                )
            ]
        )
    }

    #[test]
    fn test_url_token() {
        let lexer = Lexer::new("url(  http://www.test.lan  )");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "http://www.test.lan",
                        TokenKind::Url
                    ),
                    0..=27
                )
            ]
        )
    }

    #[test]
    fn test_bad_url_token() {
        let lexer = Lexer::new("url(  (badurl  )");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "(badurl  ",
                        TokenKind::BadUrl
                    ),
                    0..=15
                )
            ]
        )
    }

    #[test]
    fn test_function_token() {
        let lexer = Lexer::new("func(");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();   

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "func(",
                        TokenKind::Function
                    ),
                    0..=4
                )
            ]
        ) 
    }

    #[test]
    fn test_function_token_with_url() {
        let lexer = Lexer::new("url('arg')");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();   

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new("url(", TokenKind::Function),
                    0..=3
                ),
                Span::new(
                    Token::new("arg",  TokenKind::String),
                    4..=9                  
                ),
                Span::new(
                    Token::new(")", TokenKind::RightPar),
                    9..=9
                )
            ]
        ) 
    }

    #[test]
    fn test_numeric_token() {
        let lexer = Lexer::new("-12.345e-678");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();
        
        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "-12.345e-678",
                        TokenKind::Number(Number {
                            neg: true,
                            integer: Some("12"),
                            decimal: Some("345"),
                            exponent: Some(Exponent { neg: true, value: "678" })
                        })
                    ),
                    0..=11
                )
            ]
        )
    }

    #[test]
    fn test_percentage_token() {
        let lexer = Lexer::new("-12.345e-678%");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();
        
        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "-12.345e-678%",
                        TokenKind::Percentage(Number {
                            neg: true,
                            integer: Some("12"),
                            decimal: Some("345"),
                            exponent: Some(Exponent { neg: true, value: "678" })
                        })
                    ),
                    0..=12
                )
            ]
        )
    }

    #[test]
    fn test_dimension_token() {
        let lexer = Lexer::new("-12.345e-678px");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();
        
        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "-12.345e-678px",
                        TokenKind::Dimension(Dimension {
                            number: Number {
                                neg: true,
                                integer: Some("12"),
                                decimal: Some("345"),
                                exponent: Some(Exponent { neg: true, value: "678" })
                            },
                            unit: "px"
                        })
                    ),
                    0..=13
                )
            ]
        )
    }

    #[test]
    fn test_at_token() {
        let lexer = Lexer::new("@foo");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "foo",
                        TokenKind::At
                    ),
                    0..=3
                )
            ]
        )
    }

    #[test]
    fn test_hash_token() {
        let lexer = Lexer::new("#foo");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "foo",
                        TokenKind::Hash(true)
                    ),
                    0..=3
                )
            ]
        )
    }

    #[test]
    fn test_string_token() {
        let lexer = Lexer::new("'this is a string'");
        let tokens = lexer.collect::<LexerResult<Vec<_>>>().unwrap();

        assert_eq!(
            tokens,
            vec![
                Span::new(
                    Token::new(
                        "this is a string",
                        TokenKind::String
                    ),
                    0..=1
                )
            ]
        )
    }
}