use std::fmt::Display;

use crate::scanner::token::Token;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::Var(name,expr ) => write!(f, "var {} = {:?}", name.lexeme, expr),
            Stmt::Block(stmts) => {
                write!(f, "{{")?;
                for stmt in stmts {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::If(condition, then_branch, else_branch) => {
                write!(f, "if ({}) {{ {} }}", condition, then_branch)?;
                if let Some(else_branch) = else_branch {
                    write!(f, " else {{ {} }}", else_branch)
                } else {
                    Ok(())
                }
            }
        }
    }
}