// Lox expression grammar
// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;
use crate::scanner::token::{Token, TokenType};

use super::expr::Expr;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>], // slice
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Option<Vec<Expr>> {
      if self.tokens.is_empty() {
        return None
      }
      let mut exprs = vec![];
      for token in self.tokens {
        let expr = match token.token_type {
            TokenType::True => Some(Expr::Bool(true)),
            TokenType::False => Some(Expr::Bool(false)),
            TokenType::Nil => Some(Expr::Nil),
            TokenType::Number => Some(Expr::Number(token.literal.clone().unwrap())),
            TokenType::String => Some(Expr::String(token.literal.clone().unwrap())),
            _ => None,
        };
        match expr {
            Some(expr) => exprs.push(expr),
            None => ()
        }
      }
      Some(exprs)  
    }
    // fn match_tokens(&self ,tokens: &'a [Token<'a>]) -> bool {
    //     for token in tokens {
    //       if self.check(token) {
    //         self.advance();
    //         return true
    //       }
    //     }
    //     return false;
    // }
    // fn check(&self, token: &Token) -> bool {
    //   if self.is_at_end() {
    //     return false;
    //   }
    //   self.peek().token_type == token.token_type
    // }
    // fn is_at_end(&self) -> bool {
    //     self.current >= self.tokens.len()
    // }
    // fn peek(&self) -> &Token {
    //   &self.tokens[self.current]
        
    // }
    // fn expression(&self) {
    //   self.equality()
    // }
    // fn equality (&self) {
    //   let expr = self.comparison();
    // }
}
