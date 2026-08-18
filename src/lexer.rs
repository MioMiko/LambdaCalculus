use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Lambda,
    Dot,
    LParen,
    RParen,
    Eof,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    current: Token,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tmp = Self {
            input,
            chars: input.chars().peekable(),
            current: Token::new(TokenKind::Eof, Span::new(0, 0)),
            position: 0,
        };
        tmp.advance();
        tmp
    }

    pub fn peek(&self) -> &Token {
        &self.current
    }

    pub fn advance(&mut self) {
        self.consume_whitespaces();

        let start = self.position;
        let kind = match self.chars.peek().copied() {
            None => TokenKind::Eof,
            Some('λ') => {
                self.consume_char();
                TokenKind::Lambda
            }
            Some('.') => {
                self.consume_char();
                TokenKind::Dot
            }
            Some('(') => {
                self.consume_char();
                TokenKind::LParen
            }
            Some(')') => {
                self.consume_char();
                TokenKind::RParen
            }
            Some(ch) if Self::is_ident_start(ch) => {
                self.consume_char();
                self.consume_ident();
                TokenKind::Ident
            }
            _ => {
                self.consume_char();
                TokenKind::Error
            }
        };

        self.current = Token::new(kind, Span::new(start, self.position));
    }

    fn is_ident_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_ident_continue(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn consume_whitespaces(&mut self) {
        while let Some(ch) = self.chars.peek()
            && ch.is_whitespace()
        {
            self.consume_char();
        }
    }

    fn consume_char(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
        }
    }

    fn consume_ident(&mut self) {
        while let Some(&ch) = self.chars.peek()
            && Self::is_ident_continue(ch)
        {
            self.consume_char();
        }
    }

    pub fn get_str_by_span(&self, span: Span) -> &str {
        &self.input[span.start..span.end]
    }
}
