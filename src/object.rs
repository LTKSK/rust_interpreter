use std::fmt;

#[derive(Clone, Debug)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Return(Box<Object>),
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Null => "null".to_string(),
            _ => "not implemented yet".to_string(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i.to_string()),
            Self::Boolean(b) => write!(f, "{}", b.to_string()),
            Self::Return(v) => write!(f, "{}", v.as_ref()),
            Self::Null => write!(f, "null"),
        }
    }
}
