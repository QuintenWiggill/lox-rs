use std::collections::HashMap;

use crate::ast::{ Value };
use crate::scanner::{Token};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, String> {
        match self.values.get(name.lexeme.as_str()) {
            Some(val) => Ok(val.clone()),
            None => Err(format!("Undefined variable '{}'.", name.lexeme.as_str()))
        }
    }
}
