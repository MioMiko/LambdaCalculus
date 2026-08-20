use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InterpreterError {
    #[error("Syntax error: Expected {expect}, found \"{found}\" at byte index {index}")]
    UnexpectedToken {
        found: Box<str>,
        expect: &'static str,
        index: usize,
    },
}
