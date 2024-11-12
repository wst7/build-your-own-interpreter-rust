use std::{borrow::BorrowMut, fmt::{Display, Formatter}};

use crate::{
    environment::Environment,
    parser::{
        expr::{Expr, Literal},
        stmt::Stmt,
    },
    scanner::token::TokenType,
};

pub struct RuntimeError {
    message: String,
    line: usize,
}

impl RuntimeError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
pub struct Interpreter {
    env: Box<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Box::new(Environment::new(None)),
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(&stmt)?
        }
        Ok(())
    }
    // 执行语句
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Expression(expr) => {
                let value = self.evaluate(expr)?;
                // println!("{}", value);
                Ok(())
            }
            Stmt::Var(name, initializer) => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Nil,
                };
                self.env.define(name.lexeme.clone(), Some(val));
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts)?;
                Ok(())
            }
            _ => Err(RuntimeError::new("Not implemented".to_string(), 0)),
        }
    }
    fn execute_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        // let previous = self.env.clone();
        self.env = Box::new(Environment::new(Some(self.env.as_ref().clone())));
        for stmt in stmts {
            self.execute(stmt)?;
        }
        self.env = self.env.get_enclosing();
        Ok(())
    }
    // 计算表达式
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(lit) => {
                let val = match lit {
                    Literal::Number(n) => Value::Number(*n),
                    Literal::String(s) => Value::String(s.to_string()),
                    Literal::Bool(b) => Value::Bool(*b),
                    Literal::Nil => Value::Nil,
                };
                Ok(val)
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(op, expr) => {
                let right = self.evaluate(expr)?;
                match op.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = right {
                            Ok(Value::Number(-n))
                        } else {
                            Err(RuntimeError::new(
                                "Invalid operand for unary operator".to_string(),
                                op.line,
                            ))
                        }
                    }
                    TokenType::Bang => Ok(Value::Bool(!self.is_truthy(&right))),
                    _ => Ok(Value::String("Not implemented".to_string())),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = self.evaluate(left)?;
               
                let right = self.evaluate(right)?;
               
                match op.token_type {
                    TokenType::Plus => {
                        if self.is_number(&left) && self.is_number(&right) {
                            Ok(Value::Number(
                                self.get_number(&left) + self.get_number(&right),
                            ))
                        } else if self.is_string(&left) && self.is_string(&right) {
                            Ok(Value::String(format!(
                                "{}{}",
                                self.get_string(&left),
                                self.get_string(&right)
                            )))
                        } else {
                            Err(RuntimeError::new(
                                "Operands must be two numbers or two strings.".to_string(),
                                op.line,
                            ))
                        }
                    }
                    TokenType::Minus => {
                        if self.is_number(&left) && self.is_number(&right) {
                            Ok(Value::Number(
                                self.get_number(&left) - self.get_number(&right),
                            ))
                        } else {
                            Err(RuntimeError::new(
                                "Operands must be numbers.".to_string(),
                                op.line,
                            ))
                        }
                    }
                    TokenType::Star => {
                        if self.is_number(&left) && self.is_number(&right) {
                            Ok(Value::Number(
                                self.get_number(&left) * self.get_number(&right),
                            ))
                        } else {
                            Err(RuntimeError::new(
                                "Operands must be numbers.".to_string(),
                                op.line,
                            ))
                        }
                    }
                    TokenType::Slash => {
                        if self.is_number(&left) && self.is_number(&right) {
                            let right_number = self.get_number(&right);
                            if right_number == 0.0 {
                                Err(RuntimeError::new("Division by zero.".to_string(), op.line))
                            } else {
                                Ok(Value::Number(
                                    self.get_number(&left) / self.get_number(&right),
                                ))
                            }
                        } else {
                            Err(RuntimeError::new(
                                "Operands must be numbers.".to_string(),
                                op.line,
                            ))
                        }
                    }
                    TokenType::Greater => self.compare_values(&left, &right, |l, r| l > r),
                    TokenType::GreaterEqual => self.compare_values(&left, &right, |l, r| l >= r),
                    TokenType::Less => self.compare_values(&left, &right, |l, r| l < r),
                    TokenType::LessEqual => self.compare_values(&left, &right, |l, r| l <= r),
                    TokenType::EqualEqual => {
                        let result = self.compare_equality(&left, &right);
                        Ok(Value::Bool(result))
                    }
                    TokenType::BangEqual => {
                        let result = self.compare_equality(&left, &right);
                        Ok(Value::Bool(!result))
                    }
                    _ => Err(RuntimeError::new("Unimplemented".to_string(), op.line)),
                }
            }
            Expr::Variable(name) => {
              Ok(self.env.get(name)?.clone().unwrap())
            }
            Expr::Assign(name, expr) => {
              let value = self.evaluate(expr)?;
              self.env.assign(name, Some(value.clone()))?;
              Ok(value)
            }
            _ => {
              panic!("Not implemented")
            },
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
    fn is_number(&self, val: &Value) -> bool {
        matches!(val, Value::Number(_))
    }
    fn is_string(&self, val: &Value) -> bool {
        matches!(val, Value::String(_))
    }

    fn get_number(&self, val: &Value) -> f64 {
        match val {
            Value::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }
    fn get_string(&self, val: &Value) -> String {
        match val {
            Value::String(s) => s.to_string(),
            _ => panic!("Not a string"),
        }
    }
    fn compare_values<F: Fn(f64, f64) -> bool>(
        &self,
        left: &Value,
        right: &Value,
        compare: F,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(compare(*l, *r))),
            _ => Err(RuntimeError::new(
                "Operands must be numbers.".to_string(),
                0,
            )),
        }
    }

    fn compare_equality(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => (l - r).abs() < f64::EPSILON,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
