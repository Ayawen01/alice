use std::fmt::Display;

#[derive(Debug)]
pub enum AliceError {
    SyntaxError(Box<str>, u32),
    ParseError(Box<str>, u32)
}

impl Display for AliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliceError::SyntaxError(e, line) => write!(f, "line [{}] SyntaxError: {}", line, e),
            AliceError::ParseError(e, line) => write!(f, "line [{}] SyntaxError: {}", line, e)
        }
    }
}