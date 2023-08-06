use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Char(char),
    Token(String),
    Digit(usize),
    Delimiter(DelimiterKind),
    DQuote,
    CR,
    LF,
    CRLF,
    Space,
    Dot,
    Bad,
    Eof,
}

/// Enum containing the delimiters.
#[derive(Debug, PartialEq, Eq)]
pub enum DelimiterKind {
    LParen,
    RParen,
    Comma,
    Slash,
    Colon,
    Semicolon,
    GT,
    Equal,
    LT,
    QuestionMark,
    At,
    LBracket,
    Backslash,
    RBracket,
    LBrace,
    RBrace,
}

pub struct Lexer<'a> {
    peekable: &'a mut Peekable<std::slice::Iter<'a, u8>>,
}

impl<'a> Lexer<'a> {
    pub fn new(peekable: &'a mut Peekable<std::slice::Iter<'a, u8>>) -> Self {
        Self { peekable }
    }
}

trait Tokens {
    fn is_tkn(&self) -> bool;
    fn is_tchar(&self) -> bool;
    fn is_delimiter(&self) -> bool;
}

impl Tokens for u8 {
    fn is_tkn(&self) -> bool {
        self.is_ascii_alphabetic() || self.is_tchar()
    }

    fn is_tchar(&self) -> bool {
        matches!(
            &self,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
    }

    fn is_delimiter(&self) -> bool {
        todo!()
    }
}

impl Lexer<'_> {
    /// Reads a token.
    pub fn lex(&mut self) -> TokenKind {
        if let Some(u) = self.peekable.peek() {
            match u {
                b'\0' => self.just(TokenKind::Eof),
                b' ' => self.just(TokenKind::Space),
                u if u.is_tchar() => {
                    let c = self.eat().unwrap();
                    TokenKind::Char(char::from(*c))
                }
                b'"' => self.just(TokenKind::DQuote),
                b':' => self.just(TokenKind::Delimiter(DelimiterKind::Colon)),
                b'/' => self.just(TokenKind::Delimiter(DelimiterKind::Slash)),
                b'\r' => {
                    self.eat();
                    match self.peekable.peek() {
                        Some(&&b'\n') => self.just(TokenKind::CRLF),
                        _ => self.just(TokenKind::CR),
                    }
                }
                b'\n' => self.just(TokenKind::LF),
                u if u.is_ascii_uppercase() => self.accu_token(),
                u if u.is_ascii_lowercase() => self.accu_token(),
                u if u.is_ascii_digit() => self.digit(),
                _ => self.just(TokenKind::Bad),
            }
        } else {
            TokenKind::Eof
        }
    }

    /// Eats the peekable's current item.
    pub fn eat(&mut self) -> Option<&u8> {
        self.peekable.next()
    }

    /// Eats the current item and returns the `tk`.
    pub fn just(&mut self, tk: TokenKind) -> TokenKind {
        self.eat();
        tk
    }

    /// Accumulates a sequence of bytes and returns a [TokenKind::Token].
    fn accu_token(&mut self) -> TokenKind {
        let mut seq = String::new();
        while let Some(u) = self.peekable.peek() {
            if u.is_tkn() {
                let c = self.eat().unwrap();
                let c = char::from(*c);
                seq.push(c);
            } else {
                break;
            }
        }
        TokenKind::Token(seq)
    }

    /// Reads a digit and returns a [TokenKind::Digit].
    fn digit(&mut self) -> TokenKind {
        let mut digit = String::new();
        {
            let c = self.eat();
            digit.push(char::from(*c.unwrap()));
        }
        TokenKind::Digit(digit.parse().unwrap())
    }
}

impl Iterator for Lexer<'_> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.lex();
        match t {
            TokenKind::Eof => None,
            other => Some(other),
        }
    }
}

impl ToString for DelimiterKind {
    fn to_string(&self) -> String {
        match self {
            DelimiterKind::LParen => "(",
            DelimiterKind::RParen => ")",
            DelimiterKind::Comma => ",",
            DelimiterKind::Slash => "/",
            DelimiterKind::Colon => ":",
            DelimiterKind::Semicolon => ";",
            DelimiterKind::GT => ">",
            DelimiterKind::Equal => "=",
            DelimiterKind::LT => "<",
            DelimiterKind::QuestionMark => "?",
            DelimiterKind::At => "@",
            DelimiterKind::LBracket => "[",
            DelimiterKind::Backslash => "\\",
            DelimiterKind::RBracket => "]",
            DelimiterKind::LBrace => "{",
            DelimiterKind::RBrace => "}",
        }
        .to_string()
    }
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Token(tk) => return tk.clone(),
            TokenKind::Char(c) => c.to_string(),
            TokenKind::Digit(d) => d.to_string(),
            TokenKind::Delimiter(dk) => dk.to_string(),
            TokenKind::DQuote => todo!(),
            TokenKind::CR => '\r'.to_string(),
            TokenKind::LF => '\n'.to_string(),
            TokenKind::CRLF => "\r\n".to_string(),
            TokenKind::Space => ' '.to_string(),
            TokenKind::Dot => '.'.to_string(),
            TokenKind::Bad => todo!(),
            TokenKind::Eof => todo!(),
        }
    }
}
