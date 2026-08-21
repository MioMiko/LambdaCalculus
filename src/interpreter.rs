use std::{collections::HashSet, usize};

use crate::{
    Term::{self},
    error::InterpreterError,
    parser::Parser,
    string_pool::StringPool,
    term::NormalizedTerm,
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    string_pool: StringPool,
    rename_counter: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(),
            rename_counter: 0,
        }
    }

    /// Converts a term to normal form. Does eta expansion.
    pub fn normalize(&mut self, mut term: Term) -> NormalizedTerm {
        term = self.reduce(term);
        match term {
            Term::Var(var_idx) => NormalizedTerm::new_unchecked(Term::Var(var_idx)),
            Term::Lambda { param, body } => {
                let new_body = self.normalize(*body);
                if let Term::App { left, right } = new_body.as_ref()
                    && let Term::Var(var_idx) = **right
                    && param == var_idx
                    && !self.free_vars(&left).contains(&var_idx)
                {
                    let Term::App { left, .. } = new_body.into_inner() else {
                        panic!();
                    };
                    return NormalizedTerm::new_unchecked(*left);
                }
                NormalizedTerm::new_unchecked(Term::Lambda {
                    param,
                    body: new_body.into_inner().into(),
                })
            }
            Term::App { left, right } => NormalizedTerm::new_unchecked(Term::App {
                left: self.normalize(*left).into_inner().into(),
                right: self.normalize(*right).into_inner().into(),
            }),
        }
    }

    /// Evals to weak head normal form
    pub fn run(&mut self, code: &str) -> Result<Term, InterpreterError> {
        self.rename_counter = 0;
        let mut parser = Parser::new(code);
        let term = parser.parse_term(&mut self.string_pool)?;
        Ok(self.reduce(term))
    }

    fn reduce(&mut self, mut term: Term) -> Term {
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
        let free_vars = self.free_vars(pattern);
        self.apply_impl(param, body, pattern, &free_vars)
    }

    fn apply_impl(
        &mut self,
        param: usize,
        body: Term,
        pattern: &Term,
        free_vars: &HashSet<usize>,
    ) -> Term {
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
                    let (renamed_param, renamed_body) = if free_vars.contains(&body_param) {
                        let new_name = self.rename(body_param);
                        (
                            new_name,
                            self.apply(body_param, *body_body, &Term::Var(new_name)),
                        )
                    } else {
                        (body_param, *body_body)
                    };
                    Term::Lambda {
                        param: renamed_param,
                        body: self
                            .apply_impl(param, renamed_body, pattern, free_vars)
                            .into(),
                    }
                } else {
                    Term::Lambda {
                        param: body_param,
                        body: body_body,
                    }
                }
            }
            Term::App { left, right } => Term::App {
                left: self.apply_impl(param, *left, pattern, free_vars).into(),
                right: self.apply_impl(param, *right, pattern, free_vars).into(),
            },
        }
    }

    fn free_vars(&mut self, term: &Term) -> HashSet<usize> {
        let mut vars = HashSet::new();
        self.free_vars_impl(term, &mut vars);
        vars
    }

    fn free_vars_impl(&mut self, term: &Term, vars: &mut HashSet<usize>) {
        match term {
            Term::Var(var_idx) => {
                vars.insert(*var_idx);
            }
            Term::Lambda { param, body } => {
                self.free_vars_impl(body, vars);
                vars.remove(param);
            }
            Term::App { left, right } => {
                self.free_vars_impl(left, vars);
                self.free_vars_impl(right, vars);
            }
        }
    }

    pub fn equivalent(x: &NormalizedTerm, y: &NormalizedTerm) -> bool {
        Self::equivalent_impl(x, y, &mut Vec::new())
    }

    fn equivalent_impl(x: &Term, y: &Term, params: &mut Vec<(usize, usize)>) -> bool {
        match (x, y) {
            (Term::Var(v1), Term::Var(v2)) => params
                .iter()
                .rev()
                .filter(|&&(p1, p2)| p1 == *v1 || p2 == *v2)
                .map(|&(p1, p2)| p1 == *v1 && p2 == *v2)
                .next()
                .unwrap_or(*v1 == *v2),
            (
                Term::Lambda {
                    param: p1,
                    body: b1,
                },
                Term::Lambda {
                    param: p2,
                    body: b2,
                },
            ) => {
                params.push((*p1, *p2));
                let res = Self::equivalent_impl(b1, b2, params);
                params.pop();
                res
            }
            (
                Term::App {
                    left: l1,
                    right: r1,
                },
                Term::App {
                    left: l2,
                    right: r2,
                },
            ) => Self::equivalent_impl(l1, l2, params) && Self::equivalent_impl(r1, r2, params),
            _ => false,
        }
    }

    /// Temporary solution for rename
    fn rename(&mut self, origin: usize) -> usize {
        let new_name = format!(
            "{}${}",
            self.string_pool.get_str(origin).to_owned(),
            self.rename_counter
        );
        self.rename_counter += 1;
        self.string_pool.get_index(&new_name)
    }

    pub fn format_term(&self, term: &Term) -> String {
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
