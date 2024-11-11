use std::fmt::Display;

use crate::{
    parser::expr::{Expr, Literal},
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

pub struct Evaluator<'a> {
    ast: &'a Expr,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Expr) -> Self {
        Self { ast }
    }
    pub fn evaluate(&self) -> Result<Value, RuntimeError> {
        self.evaluate_expr(self.ast)
    }
    fn evaluate_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match self.ast {
            Expr::Literal(lit) => {
                let val = match lit {
                    Literal::Number(n) => Value::Number(*n),
                    Literal::String(s) => Value::String(s.to_string()),
                    Literal::Bool(b) => Value::Bool(*b),
                    Literal::Nil => Value::Nil,
                };
                Ok(val)
            }
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Unary(op, expr) => {
                let right = self.evaluate_expr(expr)?;
                match op.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = right {
                            Ok(Value::Number(-n))
                        } else {
                            Err(RuntimeError::new(
                                "Invalid operand for unary operator".to_string(),
                                0,
                            ))
                        }
                    }
                    TokenType::Bang => Ok(Value::Bool(!self.is_truthy(&right))),
                    _ => Ok(Value::String("Not implemented".to_string())),
                }
            }
            _ => panic!("Not implemented"),
        }
    }
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
}
