use std::iter::Peekable;
use std::mem::{self};

use crate::http_entity;
use crate::lexer::{Lexer, TokenKind};

#[derive(Debug)]
pub struct ParseErr(String);

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr: TokenKind,
    pub next: TokenKind,
}

impl<'a> Parser<'a> {
    pub fn new(peekable: &'a mut Peekable<std::slice::Iter<'a, u8>>) -> Self {
        let mut lexer = Lexer::new(peekable);

        let curr = lexer.lex();
        let next = lexer.lex();

        Self { lexer, curr, next }
    }
}

pub type ParseResult = Result<TokenKind, ParseErr>;

impl Parser<'_> {
    /// Advances the state of the [Lexer].
    fn walk(&mut self) -> TokenKind {
        let mut a_next = self.lexer.lex();

        mem::swap(&mut self.next, &mut self.curr);
        mem::swap(&mut self.next, &mut a_next);

        a_next
    }

    /// Returns [Ok(String)] if the current token is a [TokenKind::Token].
    fn expect_token(&mut self) -> Result<String, ParseErr> {
        match &self.curr {
            TokenKind::Token(s) => {
                let s = s.clone();
                self.walk();
                Ok(s)
            },
            _ => Err(ParseErr("Was expected a Token".into())),
        }
    }

    /// Expects the current token to be equal to `tk` and advances the parser state.
    pub fn expect(&mut self, tk: TokenKind) -> ParseResult {
        match &self.curr {
            t if *t == tk => Ok(self.walk()),
            other => Err(ParseErr(format!("Expected {:?}, found {:?}", tk, other))),
        }
    }

    /// Returns `true` if the current token is equal to `tk`.
    pub fn curr_is(&self, tk: TokenKind) -> bool {
        match &self.curr {
            t if *t == tk => true,
            _ => false,
        }
    }

    /// Walks if the current token is a [TokenKind::Space].
    pub fn opt_space(&mut self) {
        if self.curr_is(TokenKind::Space) {
            self.walk();
        }
    }
}

impl Parser<'_> {
    /// Parses a [http_entity::HttpMethod].
    pub fn method(&mut self) -> http_entity::HttpMethod {
        match self.expect_token() {
            Ok(s) => {
                match s.as_str() {
                    "GET" => http_entity::HttpMethod::GET,
                    "POST" => http_entity::HttpMethod::POST,
                    _ => http_entity::HttpMethod::BAD,
                }
            },
            Err(_) => http_entity::HttpMethod::BAD,
        }
    }

    /// Reads recursively a http path.
    pub fn path(&mut self, s: &mut String) -> Result<String, ParseErr> {
        match self.walk() {
            TokenKind::DQuote => todo!(),
            TokenKind::Space | TokenKind::CRLF | TokenKind::Eof => Ok(s.clone()),
            TokenKind::Bad => Err(ParseErr("Bad token found".into())),
            tk => {
                s.push_str(&tk.to_string());
                self.path(s)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::TokenKind;

    use super::Parser;

    #[test]
    fn fofoo() {
        let mut src = "GET /banana".as_bytes().iter().peekable();
        let mut parser = Parser::new(&mut src);

        let tk = parser.method();
        println!("tk {:?}", tk);

        parser.expect(TokenKind::Space).unwrap();

        let path = parser.path(&mut String::new());
        println!("path {:?}", path);
    }
}
