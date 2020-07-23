#[derive(Clone, Debug)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
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
