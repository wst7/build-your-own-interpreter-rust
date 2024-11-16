use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

use crate::{
    environment::Environment,
    parser::{
        expr::{Expr, Literal},
        stmt::Stmt,
    },
    scanner::token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum RuntimeError {
    Error { message: String, line: usize },
    Return(Value),
}

impl RuntimeError {
    pub fn new(message: String, line: usize) -> Self {
        Self::Error { message, line }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RuntimeError::Error { message, line } => {
                write!(f, "[line {}] Error: {}", line, message)
            }
            RuntimeError::Return(value) => {
                write!(f, "Return {}", value)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    NativeFunction(fn() -> Value),
    Function(String, Vec<Token>, Vec<Stmt>, Rc<Environment>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::NativeFunction(_) => write!(f, "<fn>"),
            Value::Function(name, _, _, _) => {
                write!(f, "<fn {}>", name)
            }
        }
    }
}
pub struct Interpreter {
    pub env: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::new(Environment::new(None));
        env.define_natives();
        Self { env: env }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(&stmt, &Rc::clone(&self.env))?
        }
        Ok(())
    }
    // 执行语句
    fn execute(&mut self, stmt: &Stmt, env: &Rc<Environment>) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr, env)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Expression(expr) => {
                let _ = self.evaluate(expr, env)?;
                Ok(())
            }
            Stmt::Var(name, initializer) => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr, env)?,
                    None => Value::Nil,
                };
                env.define(name.lexeme.clone(), Some(val));
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts, env)?;
                Ok(())
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let condition = self.evaluate(condition, env)?;
                if self.is_truthy(&condition) {
                    self.execute(then_branch, env)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch, env)?;
                }
                Ok(())
            }
            Stmt::While(condition, body) => {
                let mut condi = self.evaluate(condition, env)?;
                while self.is_truthy(&condi) {
                    self.execute(body, env)?;
                    condi = self.evaluate(condition, env)?;
                }
                Ok(())
            }
            Stmt::For(initializer, condition, increment, body) => {
                match initializer {
                    Some(stmt) => self.execute(stmt, env)?,
                    None => (),
                }
                match condition {
                    Some(expr) => {
                        let mut condi = self.evaluate(expr, env)?;
                        while self.is_truthy(&condi) {
                            self.execute(body, env)?;
                            if let Some(increment) = increment {
                                self.evaluate(increment, env)?;
                            }
                            condi = self.evaluate(expr, env)?;
                        }
                    }
                    None => {
                        self.execute(body, env)?;
                    }
                }
                Ok(())
            }
            Stmt::Function(name, params, body) => {
                let function = Value::Function(
                    name.lexeme.clone(),
                    params.clone(),
                    body.to_vec(),
                    Rc::clone(&env),
                );
                env.define(name.lexeme.clone(), Some(function));
                Ok(())
            }
            Stmt::Return(expr) => {
                let value = match expr {
                    Some(expr) => self.evaluate(expr, env)?,
                    None => Value::Nil,
                };
                Err(RuntimeError::Return(value))
            }
            _ => Err(RuntimeError::new("Not implemented".to_string(), 0)),
        }
    }
    fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: &Rc<Environment>,
    ) -> Result<(), RuntimeError> {
        let env = Rc::new(Environment::new(Some(Rc::clone(env))));
        for stmt in stmts {
            self.execute(stmt, &env)?;
        }
        Ok(())
    }
    // 计算表达式
    pub fn evaluate(&mut self, expr: &Expr, env: &Rc<Environment>) -> Result<Value, RuntimeError> {
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
            Expr::Grouping(expr) => self.evaluate(expr, env),
            Expr::Unary(op, expr) => {
                let right = self.evaluate(expr, env)?;
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
                let left = self.evaluate(left, env)?;

                let right = self.evaluate(right, env)?;

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
            Expr::Variable(name) => Ok(env.get(name)?.unwrap()),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr, env)?;
                env.assign(name, Some(value.clone()))?;
                Ok(value)
            }
            Expr::Logical(left, op, right) => {
                let left_expr = self.evaluate(left, env)?;

                // let right_expr = self.evaluate(right)?;
                match op.token_type {
                    // right  不能提前计算，可能包含Assign 表达式， 只有在left 是false时，才计算right
                    TokenType::Or => {
                        if self.is_truthy(&left_expr) {
                            return Ok(left_expr);
                        }
                        Ok(self.evaluate(right, env)?)
                    }
                    // right  不能提前计算，可能包含Assign 表达式， 只有在left 是true时，才计算right
                    TokenType::And => {
                        if !self.is_truthy(&left_expr) {
                            return Ok(left_expr);
                        }
                        Ok(self.evaluate(right, env)?)
                    }
                    _ => Err(RuntimeError::new("Not implemented".to_string(), op.line)),
                }
            }
            Expr::Call(callee, paren, arguments) => {
                let val = self.evaluate(callee, &env)?;
                match val {
                    Value::NativeFunction(func) => {
                        if !arguments.is_empty() {
                            return Err(RuntimeError::new(
                                "Native function Expected 0 arguments.".to_string(),
                                paren.line,
                            ));
                        }
                        Ok(func())
                    }
                    Value::Function(_, params, body, closure) => {
                        if arguments.len() != params.len() {
                            return Err(RuntimeError::new(
                                format!(
                                    "Expected {} arguments but got {}. ",
                                    params.len(),
                                    arguments.len()
                                ),
                                paren.line,
                            ));
                        }
                        let func_env = Rc::new(Environment::new(Some(closure.clone())));
                        for (param, arg) in params.iter().zip(arguments) {
                            // 这里花费了很多时间。。。
                            // 实参的值 必须先计算（基于函数调用时的环境），才能赋值给函数的环境
                            let value = self.evaluate(arg, &env)?;
                            func_env.define(param.lexeme.clone(), Some(value));
                        }
                        let result = self.execute_block(&body, &func_env);

                        match result {
                            Ok(_) => Ok(Value::Nil),
                            Err(RuntimeError::Return(val)) => Ok(val),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err(RuntimeError::new(
                        "Can only call functions.".to_string(),
                        paren.line,
                    )),
                }
            }
            _ => {
                panic!("Not implemented")
            }
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
