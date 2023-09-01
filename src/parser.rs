use std::collections::HashMap;
use std::iter::Peekable;
use std::mem::{self};

use crate::http_entity::{self, HttpEntity};
use crate::lexer::{DelimiterKind, Lexer, TCharKind, TokenKind};

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
            }
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
            Ok(s) => match s.as_str() {
                "GET" => http_entity::HttpMethod::GET,
                "POST" => http_entity::HttpMethod::POST,
                _ => http_entity::HttpMethod::BAD,
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
            }
        }
    }

    /// Parses a http 1.1 version.
    pub fn http_1_1(&mut self) -> Result<http_entity::HttpVsn, ParseErr> {
        use TokenKind::*;

        let http = self.expect_token()?;

        println!("{:?}", &http);
        if !(http == "HTTP") {
            return Err(ParseErr("Expected 'HTTP' token".into()));
        }

        let _slash = self.expect(Delimiter(DelimiterKind::Slash))?;

        let _one = self.expect(Char(TCharKind::Digit(1)))?;
        let _dot = self.expect(Char(TCharKind::Dot))?;
        let _one = self.expect(Char(TCharKind::Digit(1)))?;

        Ok(http_entity::HttpVsn::HTTP1_1)
    }

    /// Parses a http request line.
    ///
    /// # Example
    ///
    /// ```
    /// GET /items HTTP/1.1\r\n
    /// ```
    pub fn request_line(&mut self) -> Result<RequestLine, ParseErr> {
        let method = self.method();
        self.expect(TokenKind::Space)?;
        let path = self.path(&mut String::new())?;
        let http_version = self.http_1_1()?;
        self.expect(TokenKind::CRLF)?;

        Ok(RequestLine(method, path, http_version))
    }

    pub fn headers(&mut self) -> Result<HttpEntity, ParseErr> {
        let request_line = self.request_line()?;

        let mut headers = HashMap::<String, String>::new();

        while !self.curr_is(TokenKind::CRLF) {
            let tk = self.expect_token()?;
            self.expect(TokenKind::Delimiter(DelimiterKind::Colon))?;
            self.opt_space();

            let mut val = String::new();

            while !self.curr_is(TokenKind::CRLF) {
                let str_tk = self.walk().to_string();
                val.push_str(&str_tk);
            }

            self.expect(TokenKind::CRLF)?;

            headers.insert(tk, val);
        }

        let http_entity = HttpEntity {
            method: request_line.0,
            http_version: request_line.2,
            path: request_line.1,
            headers,
        };
        Ok(http_entity)
    }
}

#[derive(Debug)]
pub struct RequestLine(http_entity::HttpMethod, String, http_entity::HttpVsn);

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
