use std::fmt::{Display, Formatter};

use crate::scanner::token::Token;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(l) => write!(f, "{}", l),
            Expr::Unary(op, e) => write!(f, "{} {}", op.lexeme, e),
            Expr::Binary(l, op, r) => write!(f, "{} {l} {r}", op.lexeme),
            Expr::Grouping(g) => write!(f, "(group {})", g),
        }
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => {
                let mut value = n.to_string();
                if !value.contains(".") {
                    value.push_str(".0");
                }
                write!(f, "{}", value)
            }
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
