use crate::ast::{Expression, Statement};
use crate::builtins::BuiltinError;
use crate::environment::Environment;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
        env: Environment,
    },
    Array(Vec<Object>),
    Builtin(fn(Vec<Object>) -> Result<Object, BuiltinError>),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i.to_string()),
            Self::String(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{}", b.to_string()),
            Self::Return(v) => write!(f, "{}", v.as_ref()),
            Self::Null => write!(f, "null"),
            Self::Function { .. } => write!(f, ""),
            Self::Array(elements) => {
                let mut s = String::from("");
                s.push_str("[");
                s.push_str(
                    &elements
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<_>>()
                        .join(","),
                );
                s.push_str("]");
                write!(f, "{}", s)
            }
            Self::Builtin(_) => write!(f, "builtin function"),
        }
    }
}
