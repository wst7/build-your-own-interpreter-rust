use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, Value},
    scanner::token::Token,
};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Value>>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.map(|env| Box::new(env)),
        }
    }
    pub fn define(&mut self, name: String, value: Option<Value>) {
        // println!("define {} = {}", name, value.clone().unwrap());
        self.values.insert(name, value);
    }
    pub fn assign(&mut self, name: &Token, value: Option<Value>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            // println!("assign{} = {}", name.lexeme, value.clone().unwrap());
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        } else if self.enclosing.is_some() {
            return self.enclosing.as_mut().unwrap().assign(name, value);
        } else {
            Err(RuntimeError::new(
                format!("Undefined variable '{}'.", &name.lexeme),
                name.line,
            ))
        }
    }
    pub fn get(&self, name: &Token) -> Result<&Option<Value>, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap());
        }
        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(name);
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", &name.lexeme),
            name.line,
        ))
    }
    pub fn get_enclosing(&self, ) -> Box<Environment>{
        self.enclosing.as_ref().unwrap().clone()
    }
}
