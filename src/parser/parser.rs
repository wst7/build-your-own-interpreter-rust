// Grammar in grammar.txt file
use crate::scanner::token::{Token, TokenType};

use super::{
    error::ParseError,
    expr::{self, Expr, Literal},
    stmt::Stmt,
};

pub struct Parser<'a> {
    tokens: &'a [Token], // slice
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        return Ok(Stmt::Var(name, initializer));
    }
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }
        self.expression_stmt()
    }
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if'.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'while'.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While(condition, body))
    }
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(Box::new(self.var_declaration()?))
        } else {
            Some(Box::new(self.expression_stmt()?))
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            Some(Expr::Literal(Literal::Bool(true))) // Default to true if no condition
        };

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;
       
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::For(initializer, condition, increment, body))
    }
    fn expression_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        }
        Ok(Stmt::Expression(expr))
    }
    // *******解析器处理表达式时，优先从低优先级的运算符解析到高优先级的运算符************
    // expression     → assignment ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }
    // assignment     → IDENTIFIER "=" assignment | anonFunc | logic_or ;
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }
            return Err(ParseError::new("Invalid assignment target.", equals.line));
        }
        Ok(expr)
    }
    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }
    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
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
        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        } else if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        } else {
            self.literal()
        }
    }
    fn literal(&mut self) -> Result<Expr, ParseError> {
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
        Err(ParseError::new("Expect expression.", self.peek().line))
    }
    // *******辅助方法************
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type.clone()) {
            return Ok(self.advance());
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
        self.peek().token_type == TokenType::Eof
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

// pub fn print_ast(expr: &Expr) -> String {
//     match expr {
//         Expr::Literal(l) => match l {
//             Literal::Number(n) => {
//                 let mut value = n.to_string();
//                 if !value.contains(".") {
//                     value.push_str(".0");
//                 }
//                 format!("{}", l)
//             }
//             _ => format!("{}", l),
//         },
//         Expr::Unary(op, e) => format!("({} {})", op.lexeme, print_ast(e)),
//         Expr::Binary(left, op, right) => {
//             format!("({} {} {})", op.lexeme, print_ast(left), print_ast(right))
//         }
//         Expr::Grouping(g) => format!("(group {})", print_ast(g)),
//     }
// }
