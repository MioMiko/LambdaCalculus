#[derive(Debug, Clone)]
pub enum Term {
    Var(usize),
    Lambda { param: usize, body: Box<Term> },
    App { left: Box<Term>, right: Box<Term> },
}
