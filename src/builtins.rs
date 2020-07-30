use crate::object::Object;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuiltinError {
    pub msg: String,
}

fn len(args: Vec<Object>) -> Result<Object, BuiltinError> {
    if args.len() != 1 {
        return Err(BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i32)),
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
