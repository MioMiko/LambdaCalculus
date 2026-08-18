use crate::{Term, parser::Parser, string_pool::StringPool};

#[derive(Debug, Clone)]
pub struct Interpreter {
    string_pool: StringPool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(),
        }
    }

    pub fn run(&mut self, code: &str) {
        let mut parser = Parser::new(code);
        let mut term = parser.parse_term(&mut self.string_pool);
        println!("{:?}", term);
        term = self.reduce(term);
        println!("{}", self.format_term(&term));
    }

    fn reduce(&mut self, mut term: Term) -> Term {
        // println!("Try reduce: {}", self.format_term(&term));
        while let Term::App { left, right } = term {
            let left = self.reduce(*left);
            let right = self.reduce(*right);
            term = if let Term::Lambda { param, body } = left {
                self.apply(param, *body, &right)
            } else {
                return Term::App {
                    left: left.into(),
                    right: right.into(),
                };
            }
        }
        term
    }

    fn apply(&mut self, param: usize, body: Term, pattern: &Term) -> Term {
        match body {
            Term::Var(var_idx) => {
                if var_idx == param {
                    pattern.clone()
                } else {
                    body
                }
            }
            Term::Lambda {
                param: body_param,
                body: body_body,
            } => {
                if param != body_param {
                    Term::Lambda {
                        param: body_param,
                        body: self.apply(param, *body_body, pattern).into(),
                    }
                } else {
                    Term::Lambda {
                        param: body_param,
                        body: body_body,
                    }
                }
            }
            Term::App { left, right } => Term::App {
                left: self.apply(param, *left, pattern).into(),
                right: self.apply(param, *right, pattern).into(),
            },
        }
    }

    fn format_term(&self, term: &Term) -> String {
        let mut s = String::new();
        self.format_term_impl(term, false, &mut s);
        s
    }

    fn format_term_impl(&self, term: &Term, paren: bool, s: &mut String) {
        match term {
            Term::Var(string_idx) => {
                s.push_str(self.string_pool.get_str(*string_idx));
            }
            Term::Lambda { param, body } => {
                if paren {
                    s.push('(');
                }
                s.push('λ');
                s.push_str(self.string_pool.get_str(*param));
                s.push('.');
                self.format_term_impl(body, false, s);
                if paren {
                    s.push(')');
                }
            }
            Term::App { left, right } => {
                if paren {
                    s.push('(');
                }
                self.format_term_impl(left, true, s);
                s.push(' ');
                self.format_term_impl(right, true, s);
                if paren {
                    s.push(')');
                }
            }
        }
    }
}
