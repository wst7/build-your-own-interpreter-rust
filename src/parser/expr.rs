use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub enum Expr {
  Bool(bool),
  Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Nil => write!(f, "nil"),
        }
    }
}