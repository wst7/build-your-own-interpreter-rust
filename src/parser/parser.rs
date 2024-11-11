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

// 优先级
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
use crate::scanner::token::{Token, TokenType};

use super::{
    error::ParseError,
    expr::{Expr, Literal},
};

pub struct Parser<'a> {
    tokens: &'a [Token], // slice
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
    // *******解析器处理表达式时，优先从低优先级的运算符解析到高优先级的运算符************
    // expression     → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }
    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            return Ok(Expr::Literal(
                self.convert_token_literal(self.previous().clone())?,
            ));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(ParseError::new("Expect expression.", self.peek().line))
    }

    // *******辅助方法************
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(token_type.clone()) {
            self.advance();
            return Ok(());
        }
        return Err(ParseError::new(message, self.peek().line));
    }
    // 只要有一个匹配的，就调一下advance，返回true
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    // 移动指针，并且返回前一个token
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn convert_token_literal(&self, token: Token) -> Result<Literal, ParseError> {
        let literal = match token.token_type {
            TokenType::False => Literal::Bool(false),
            TokenType::True => Literal::Bool(true),
            TokenType::Nil => Literal::Nil,
            TokenType::Number => {
                if let Some(literal) = token.literal {
                    let value = match literal.parse::<f64>() {
                        Ok(value) => value,
                        Err(_) => return Err(ParseError::new("Expect number.", token.line)),
                    };
                    Literal::Number(value)
                } else {
                    return Err(ParseError::new("Expect number.", token.line));
                }
            }
            TokenType::String => {
                if let Some(literal) = token.literal {
                    Literal::String(literal)
                } else {
                    return Err(ParseError::new("Expect string.", token.line));
                }
            }
            _ => return Err(ParseError::new("Expect literal.", token.line)),
        };
        Ok(literal)
    }
}
