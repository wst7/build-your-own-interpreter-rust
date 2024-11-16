use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    interpreter::{RuntimeError, Value},
    scanner::token::Token,
};

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: RefCell<HashMap<String, Option<Value>>>,
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<Environment>>) -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            enclosing: enclosing.map(|env| env),
        }
    }
    pub fn define(&self, name: String, value: Option<Value>) {
        self.values.borrow_mut().insert(name, value);
    }
    pub fn assign(&self, name: &Token, value: Option<Value>) -> Result<(), RuntimeError> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values.borrow_mut().insert(name.lexeme.clone(), value);
            return Ok(());
        } else if let Some(enclosing) = &self.enclosing {
            return enclosing.assign(name, value);
        } else {
            Err(RuntimeError::new(
                format!("Undefined variable '{}'.", &name.lexeme),
                name.line,
            ))
        }
    }
    pub fn get(&self, name: &Token) -> Result<Option<Value>, RuntimeError> {
        if let Some(value) = self.values.borrow().get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name); // 递归查找父作用域
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", &name.lexeme),
            name.line,
        ))
    }

    pub fn define_natives(&self) {
        self.define(
            "clock".to_string(),
            Some(Value::NativeFunction(|| {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();
                Value::Number(now.as_secs_f64())
            })),
        );
    }
}
