use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub enum Expr {
  Bool(bool),
  Nil,
  Number(String),
  String(String),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "{}", s),
        }
    }
}