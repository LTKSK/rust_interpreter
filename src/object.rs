use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Return(Box<Object>),
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
        env: Environment,
    },
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i.to_string()),
            Self::Boolean(b) => write!(f, "{}", b.to_string()),
            Self::Return(v) => write!(f, "{}", v.as_ref()),
            Self::Null => write!(f, "null"),
            Self::Function { .. } => write!(f, ""),
            _ => write!(f, "not implemented yet"),
        }
    }
}
