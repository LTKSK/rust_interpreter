use crate::object::Object;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct BuiltinError {
    pub msg: String,
}

impl fmt::Display for BuiltinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BuiltinError: {}", self.msg)
    }
}

fn len(args: Vec<Object>) -> Result<Object, BuiltinError> {
    if args.len() != 1 {
        return Err(BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i32)),
        Object::Array(arr) => Ok(Object::Integer(arr.len() as i32)),
        _ => Err(BuiltinError {
            msg: "`len` support String only".to_string(),
        }),
    }
}

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert("len".to_string(), Object::Builtin(len));
    builtins
}
