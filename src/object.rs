use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;

// Objectの中でKeyとして使えるものを抽出する
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MapKey {
    Integer(i32),
    Boolean(bool),
    String(String),
    Null,
}

impl fmt::Display for MapKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Null => write!(f, "null"),
        }
    }
}

impl From<Object> for MapKey {
    fn from(item: Object) -> Self {
        match item {
            Object::Integer(i) => MapKey::Integer(i),
            Object::String(s) => MapKey::String(s),
            Object::Boolean(b) => MapKey::Boolean(b),
            Object::Null => MapKey::Null,
            _ => MapKey::Null,
        }
    }
}

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
    Builtin(fn(Vec<Object>) -> Result<Object, Error>),
    Map(HashMap<MapKey, Box<Object>>),
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
            Self::Map(m) => {
                let mut s = String::from("");
                s.push_str("{");
                s.push_str(
                    &m.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join(","),
                );
                s.push_str("}");
                write!(f, "{}", s)
            }
            Self::Builtin(_) => write!(f, "builtin function"),
        }
    }
}
