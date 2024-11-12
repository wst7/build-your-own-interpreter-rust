use std::fmt::{Display, Formatter};

use crate::{
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
pub fn evaluate_stmt(stmt: &Stmt) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Print(expr) => {
            let value = evaluate_expr(expr)?;
            println!("{}", value);
            Ok(())
        }
        Stmt::Expression(expr) => {
            let value = evaluate_expr(expr)?;
            // println!("{}", value);
            Ok(())
        }
        _ => Err(RuntimeError::new("Not implemented".to_string(), 0)),
    }
}

pub fn evaluate_expr(expr: &Expr) -> Result<Value, RuntimeError> {
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
        Expr::Grouping(expr) => evaluate_expr(expr),
        Expr::Unary(op, expr) => {
            let right = evaluate_expr(expr)?;
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
                TokenType::Bang => Ok(Value::Bool(!is_truthy(&right))),
                _ => Ok(Value::String("Not implemented".to_string())),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = evaluate_expr(left)?;

            let right = evaluate_expr(right)?;
            match op.token_type {
                TokenType::Plus => {
                    if is_number(&left) && is_number(&right) {
                        Ok(Value::Number(get_number(&left) + get_number(&right)))
                    } else if is_string(&left) && is_string(&right) {
                        Ok(Value::String(format!(
                            "{}{}",
                            get_string(&left),
                            get_string(&right)
                        )))
                    } else {
                        Err(RuntimeError::new(
                            "Operands must be two numbers or two strings.".to_string(),
                            op.line,
                        ))
                    }
                }
                TokenType::Minus => {
                    if is_number(&left) && is_number(&right) {
                        Ok(Value::Number(get_number(&left) - get_number(&right)))
                    } else {
                        Err(RuntimeError::new(
                            "Operands must be numbers.".to_string(),
                            op.line,
                        ))
                    }
                }
                TokenType::Star => {
                    if is_number(&left) && is_number(&right) {
                        Ok(Value::Number(get_number(&left) * get_number(&right)))
                    } else {
                        Err(RuntimeError::new(
                            "Operands must be numbers.".to_string(),
                            op.line,
                        ))
                    }
                }
                TokenType::Slash => {
                    if is_number(&left) && is_number(&right) {
                        let right_number = get_number(&right);
                        if right_number == 0.0 {
                            Err(RuntimeError::new("Division by zero.".to_string(), op.line))
                        } else {
                            Ok(Value::Number(get_number(&left) / get_number(&right)))
                        }
                    } else {
                        Err(RuntimeError::new(
                            "Operands must be numbers.".to_string(),
                            op.line,
                        ))
                    }
                }
                TokenType::Greater => compare_values(&left, &right, |l, r| l > r),
                TokenType::GreaterEqual => compare_values(&left, &right, |l, r| l >= r),
                TokenType::Less => compare_values(&left, &right, |l, r| l < r),
                TokenType::LessEqual => compare_values(&left, &right, |l, r| l <= r),
                TokenType::EqualEqual => {
                    let result = compare_equality(&left, &right);
                    Ok(Value::Bool(result))
                }
                TokenType::BangEqual => {
                    let result = compare_equality(&left, &right);
                    Ok(Value::Bool(!result))
                }
                _ => Err(RuntimeError::new("Unimplemented".to_string(), op.line)),
            }
        }
        _ => panic!("Not implemented"),
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Nil => false,
        _ => true,
    }
}
fn is_number(val: &Value) -> bool {
    matches!(val, Value::Number(_))
}
fn is_string(val: &Value) -> bool {
    matches!(val, Value::String(_))
}

fn get_number(val: &Value) -> f64 {
    match val {
        Value::Number(n) => *n,
        _ => panic!("Not a number"),
    }
}
fn get_string(val: &Value) -> String {
    match val {
        Value::String(s) => s.to_string(),
        _ => panic!("Not a string"),
    }
}
fn compare_values<F: Fn(f64, f64) -> bool>(
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

fn compare_equality(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => (l - r).abs() < f64::EPSILON,
        (Value::String(l), Value::String(r)) => l == r,
        (Value::Bool(l), Value::Bool(r)) => l == r,
        (Value::Nil, Value::Nil) => true,
        _ => false,
    }
}
