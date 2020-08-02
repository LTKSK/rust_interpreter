use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError { msg: String },
    EvalError { msg: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError { msg } => write!(f, "ParseError: {}", msg),
            Self::EvalError { msg } => write!(f, "EvalError: {}", msg),
        }
    }
}
