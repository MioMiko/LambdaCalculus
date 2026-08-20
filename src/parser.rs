use crate::{
    Term,
    error::InterpreterError,
    lexer::{Lexer, Token, TokenKind},
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

    pub fn parse_term(&mut self, string_pool: &mut StringPool) -> Result<Term, InterpreterError> {
        let term = self.parse_app(string_pool)?;
        let token = self.lexer.peek();
        if token.kind() != TokenKind::Eof {
            return Err(self.get_unexpected_token_err(token, "identifier, 'λ', '('"));
        }
        Ok(term)
    }

    fn parse_atom(&mut self, string_pool: &mut StringPool) -> Result<Term, InterpreterError> {
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
                    return Err(self.get_unexpected_token_err(next_token, "identifier"));
                }
                let id = self.lexer.get_str_by_span(next_token.span());
                let string_idx = string_pool.get_index(id);
                self.lexer.advance();
                let next_token = self.lexer.peek();
                if next_token.kind() != TokenKind::Dot {
                    return Err(self.get_unexpected_token_err(next_token, "'.'"));
                }
                self.lexer.advance();
                Term::Lambda {
                    param: string_idx,
                    body: self.parse_app(string_pool)?.into(),
                }
            }
            TokenKind::LParen => {
                self.lexer.advance();
                let tmp = self.parse_app(string_pool)?;
                let next_token = self.lexer.peek();
                if next_token.kind() != TokenKind::RParen {
                    return Err(self.get_unexpected_token_err(next_token, "')'"));
                }
                self.lexer.advance();
                tmp
            }
            _ => {
                return Err(self.get_unexpected_token_err(token, "identifier, 'λ', '('"));
            }
        };
        Ok(term)
    }

    fn parse_app(&mut self, string_pool: &mut StringPool) -> Result<Term, InterpreterError> {
        let mut left = self.parse_atom(string_pool)?;
        while Self::is_atom_start(self.lexer.peek().kind()) {
            left = Term::App {
                left: left.into(),
                right: self.parse_atom(string_pool)?.into(),
            }
        }
        Ok(left)
    }

    fn is_atom_start(kind: TokenKind) -> bool {
        kind == TokenKind::Lambda || kind == TokenKind::LParen || kind == TokenKind::Ident
    }

    fn get_unexpected_token_err(&self, token: &Token, expect: &'static str) -> InterpreterError {
        InterpreterError::UnexpectedToken {
            found: self.lexer.get_str_by_span(token.span()).into(),
            expect,
            index: token.span().start,
        }
    }
}
