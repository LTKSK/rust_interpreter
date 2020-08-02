use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;

fn len(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i32)),
        Object::Array(arr) => Ok(Object::Integer(arr.len() as i32)),
        _ => Err(Error::BuiltinError {
            msg: "`len` support String only".to_string(),
        }),
    }
}

fn first(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::Array(arr) => match arr.get(0) {
            Some(value) => Ok(value.clone()),
            None => Ok(Object::Null),
        },
        _ => Err(Error::BuiltinError {
            msg: format!("argument to `first` must be Array, got {:?}", &args[0]),
        }),
    }
}

fn last(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::Array(arr) => match arr.get(arr.len() - 1) {
            Some(value) => Ok(value.clone()),
            None => Ok(Object::Null),
        },
        _ => Err(Error::BuiltinError {
            msg: format!("argument to `last` must be Array, got {:?}", &args[0]),
        }),
    }
}

fn rest(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=1", args.len()),
        });
    }

    match &args[0] {
        Object::Array(arr) => {
            if arr.len() == 0 {
                return Ok(Object::Null);
            }
            Ok(Object::Array(arr[1..].to_vec()))
        }
        _ => Err(Error::BuiltinError {
            msg: format!("argument to `rest` must be Array, got {:?}", &args[0]),
        }),
    }
}

fn push(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 2 {
        return Err(Error::BuiltinError {
            msg: format!("wrong number of arguments. got={}, want=2", args.len()),
        });
    }

    match (&args[0], &args[1]) {
        (Object::Array(arr), obj) => {
            let mut arr = arr.clone();
            arr.push(obj.clone());
            Ok(Object::Array(arr.to_vec()))
        }
        _ => Err(Error::BuiltinError {
            msg: format!("first argument to `push` must be Array, got {:?}", &args[0]),
        }),
    }
}

fn puts(args: Vec<Object>) -> Result<Object, Error> {
    for a in args {
        println!("{}", a);
    }
    Ok(Object::Null)
}

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert("len".to_string(), Object::Builtin(len));
    builtins.insert("first".to_string(), Object::Builtin(first));
    builtins.insert("last".to_string(), Object::Builtin(last));
    builtins.insert("rest".to_string(), Object::Builtin(rest));
    builtins.insert("push".to_string(), Object::Builtin(push));
    builtins.insert("puts".to_string(), Object::Builtin(puts));
    builtins
}
