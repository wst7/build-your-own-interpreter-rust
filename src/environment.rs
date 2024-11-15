use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::{RuntimeError, Value},
    scanner::token::Token,
};

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Option<Value>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}
// Rc<RefCell<Environment>>
// 在解释器中，环境（Environment）通常表示一个作用域，存储变量的定义和值。不同作用域可能会嵌套在一起，比如一个函数作用域可能包含在全局作用域内。在 Rust 中，为了实现这样的嵌套关系，Environment 结构体中的 enclosing 引用需要：
// 1.	能够被多个 Environment 所共享。
// 2.	允许在共享的 Environment 中进行可变更改，例如定义和修改变量。

// 这正是 Rc<RefCell<T>> 适合的场景。
// 为什么选择 Rc<RefCell<Environment>>

// 	1.	Rc（引用计数）：
// 	•	Rc（Reference Counted）允许多个作用域共享同一个父环境。
// 	•	Rc 的引用计数会在最后一个引用消失时自动清理内存，适合单线程环境。
// 	•	在解释器中，通常每个作用域会有自己的 Environment 实例，可能会引用其父级 Environment，因此需要共享所有权。
// 	2.	RefCell（内部可变性）：
// 	•	RefCell 允许在一个不可变的 Rc 包裹中进行可变借用。
// 	•	这使得在 Rc<RefCell<Environment>> 中，可以安全地借用父环境（enclosing），并允许在解释期间修改 Environment 的内容。

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: match enclosing {
                Some(enclosing) => Some(enclosing),
                None => None,
            },
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
        } else if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        } else {
            Err(RuntimeError::new(
                format!("Undefined variable '{}'.", &name.lexeme),
                name.line,
            ))
        }
    }
    pub fn get(&self, name: &Token) -> Result<Option<Value>, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name); // 递归查找父作用域
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", &name.lexeme),
            name.line,
        ))
    }
    pub fn get_enclosing(&self) -> Rc<RefCell<Environment>> {
        self.enclosing.as_ref().unwrap().clone()
    }

    pub fn define_natives(&mut self) {
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
