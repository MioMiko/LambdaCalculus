use std::ops::Deref;

/// Term doesn't store any environmental data,
/// so a Term created by an interpreter can only be used in that specific interpreter.
#[derive(Debug, Clone)]
pub enum Term {
    Var(usize),
    Lambda { param: usize, body: Box<Term> },
    App { left: Box<Term>, right: Box<Term> },
}

#[derive(Debug, Clone)]
pub struct NormalizedTerm(Term);

impl NormalizedTerm {
    pub fn new_unchecked(term: Term) -> Self {
        Self(term)
    }

    pub fn into_inner(self) -> Term {
        self.0
    }
}

impl Deref for NormalizedTerm {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Term> for NormalizedTerm {
    fn as_ref(&self) -> &Term {
        &self.0
    }
}
