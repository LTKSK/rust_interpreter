use crate::object::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: &Environment) -> Self {
        let mut env = Self::new();
        env.outer = Some(Box::new(outer.clone()));
        return env;
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        match self.store.get(name) {
            Some(v) => Some(v),
            None => match &self.outer {
                Some(e) => e.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}
