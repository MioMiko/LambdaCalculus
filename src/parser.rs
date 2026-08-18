use crate::{
    Term,
    lexer::{Lexer, TokenKind},
    string_pool::StringPool,
};

#[derive(Debug, Clone)]
pub struct Parser<'lexer> {
    lexer: Lexer<'lexer>,
}

impl<'lexer> Parser<'lexer> {
    pub fn new(code: &'lexer str) -> Self {
        Self {
            lexer: Lexer::new(code),
        }
    }

    pub fn parse_term(&mut self, string_pool: &mut StringPool) -> Term {
        let term = self.parse_app(string_pool);
        let token = self.lexer.peek();
        if token.kind() != TokenKind::Eof {
            panic!(
                "Unexpected trailing token '{}' at byte index {}",
                self.lexer.get_str_by_span(token.span()),
                token.span().start
            );
        }
        term
    }

    fn parse_atom(&mut self, string_pool: &mut StringPool) -> Term {
        let token = self.lexer.peek();
        let term = match token.kind() {
            TokenKind::Ident => {
                let id = self.lexer.get_str_by_span(token.span());
                let term = Term::Var(string_pool.get_index(id));
                self.lexer.advance();
                term
            }
            TokenKind::Lambda => {
                self.lexer.advance();
                let next_token = self.lexer.peek();
                if next_token.kind() != TokenKind::Ident {
                    panic!(
                        "Expect Identifiner, But Found \"{}\" At Byte Index {}",
                        self.lexer.get_str_by_span(next_token.span()),
                        next_token.span().start
                    );
                }
                let id = self.lexer.get_str_by_span(next_token.span());
                let string_idx = string_pool.get_index(id);
                self.lexer.advance();
                let next_token = self.lexer.peek();
                if next_token.kind() != TokenKind::Dot {
                    panic!(
                        "Expect '.', But Found \"{}\" At Byte Index {}",
                        self.lexer.get_str_by_span(next_token.span()),
                        next_token.span().start
                    );
                }
                self.lexer.advance();
                Term::Lambda {
                    param: string_idx,
                    body: self.parse_app(string_pool).into(),
                }
            }
            TokenKind::LParen => {
                self.lexer.advance();
                let tmp = self.parse_app(string_pool);
                let next_token = self.lexer.peek();
                if next_token.kind() != TokenKind::RParen {
                    panic!(
                        "Expect ')', But Found \"{}\" At Byte Index {}",
                        self.lexer.get_str_by_span(next_token.span()),
                        next_token.span().start
                    );
                }
                self.lexer.advance();
                tmp
            }
            _ => {
                panic!(
                    "Unexpected Token \"{}\" At Byte Index {}",
                    self.lexer.get_str_by_span(token.span()),
                    token.span().start
                )
            }
        };
        term
    }

    fn parse_app(&mut self, string_pool: &mut StringPool) -> Term {
        let mut left = self.parse_atom(string_pool);
        while Self::is_atom_start(self.lexer.peek().kind()) {
            left = Term::App {
                left: left.into(),
                right: self.parse_atom(string_pool).into(),
            }
        }
        left
    }

    fn is_atom_start(kind: TokenKind) -> bool {
        kind == TokenKind::Lambda || kind == TokenKind::LParen || kind == TokenKind::Ident
    }
}
