use std::collections::HashMap;

use crate::{interpreter::{RuntimeError, Value}, scanner::token::Token};



pub struct Environment {
  values: HashMap<String, Option<Value>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: String, value: Option<Value>) {
      self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<&Option<Value>, RuntimeError> {
      if self.values.contains_key(&name.lexeme) {
        Ok(self.values.get(&name.lexeme).unwrap())
      } else {
        Err(RuntimeError::new(format!("Undefined variable '{}'.", &name.lexeme), 0))
      }
    }
}