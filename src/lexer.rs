use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Char(TCharKind),
    Token(String),
    Delimiter(DelimiterKind),
    DQuote,
    CR,
    LF,
    CRLF,
    Space,
    Bad,
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TCharKind {
    ExclamationMark,  // !
    Hash,             // #
    DollarSign,       // $
    Percent,          // %
    And,              // &
    SQuote,           // '
    Star,             // *
    Plus,             // +
    Min,              // -
    Dot,              // .
    Circumflex,       // ^
    Underscore,       // _
    Backquote,        // `
    Pipe,             // |
    Tilde,            // ~
    Digit(u8),        // 0..9
    Alpha(char),      // a..z0..9
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
        use TokenKind::*;

        if let Some(u) = self.peekable.peek() {
            match u {
                b'\0' => self.just(Eof),
                b' ' => self.just(Space),
                
                b'!' => self.just(Char(TCharKind::ExclamationMark)),
                b'#' => self.just(Char(TCharKind::Hash)),
                b'$' => self.just(Char(TCharKind::DollarSign)),
                b'%' => self.just(Char(TCharKind::Percent)),
                b'&' => self.just(Char(TCharKind::And)),
                b'\'' => self.just(Char(TCharKind::SQuote)),
                b'*' => self.just(Char(TCharKind::Star)),
                b'+' => self.just(Char(TCharKind::Plus)),
                b'-' => self.just(Char(TCharKind::Min)),
                b'.' => self.just(Char(TCharKind::Dot)),
                b'^' => self.just(Char(TCharKind::Circumflex)),
                b'_' => self.just(Char(TCharKind::Underscore)),
                b'`' => self.just(Char(TCharKind::Backquote)),
                b'|' => self.just(Char(TCharKind::Pipe)),
                b'~' => self.just(Char(TCharKind::Tilde)),

                b'"' => self.just(DQuote),
                b':' => self.just(Delimiter(DelimiterKind::Colon)),
                b'/' => self.just(Delimiter(DelimiterKind::Slash)),

                b'\r' => {
                    self.eat();
                    match self.peekable.peek() {
                        Some(&&b'\n') => self.just(CRLF),
                        _ => self.just(CR),
                    }
                },
                b'\n' => self.just(LF),
                
                u if u.is_ascii_digit() => self.digit(),
                u if u.is_ascii_alphanumeric() => {
                    match self.peekable.peek() {
                        Some(v) if v.is_ascii_alphanumeric() => self.accu_token(),
                        Some(_) | None => {
                            let a = char::from(*self.eat().unwrap());
                            self.just(Char(TCharKind::Alpha(a)))
                        }
                    }
                },
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

    /// Reads a digit and returns a [TokenKind::Char] of [TCharKind::Digit].
    fn digit(&mut self) -> TokenKind {
        let mut digit = String::new();
        {
            let c = self.eat();
            digit.push(char::from(*c.unwrap()));
        }
        TokenKind::Char(TCharKind::Digit(digit.parse().unwrap()))
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
            TokenKind::Char(_c) => "".into(),
            // TokenKind::Digit(d) => d.to_string(),
            TokenKind::Delimiter(dk) => dk.to_string(),
            TokenKind::DQuote => todo!(),
            TokenKind::CR => '\r'.to_string(),
            TokenKind::LF => '\n'.to_string(),
            TokenKind::CRLF => "\r\n".to_string(),
            TokenKind::Space => ' '.to_string(),
            // TokenKind::Dot => '.'.to_string(),
            TokenKind::Bad => todo!(),
            TokenKind::Eof => todo!(),
        }
    }
}
