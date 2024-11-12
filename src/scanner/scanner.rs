use crate::parser::expr::Literal;

use super::{
    keywords,
    token::{Error, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<Error>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<Error>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        (&self.tokens, &self.errors)
    }

    // 是否到达了文件的结尾
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => return,
        };
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '/' => {
                // comment
                if self.next_char_match('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '!' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => {
                self.errors.push(Error {
                    line: self.line,
                    message: format!("Unexpected character: {}", c),
                });
            }
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }
    // single-character tokens
    pub fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        // let text = &self.source[self.start..self.current];
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();
        self.tokens.push(Token::new(
            token_type,
            String::from(text),
            literal,
            self.line,
        ));
    }
    fn identifier(&mut self) {
        loop {
            let c = self.peek();
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let text = &self.source[self.start..self.current];
        let keyword = keywords::map().get(text);
        if let Some(token_type) = keyword {
            self.add_token(*token_type, None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }

    fn next_char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current).unwrap();
        if c != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => '\n',
        }
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\n';
        }
        match self.source.chars().nth(self.current + 1) {
            Some(c) => c,
            None => '\n',
        }
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.errors.push(Error {
                line: self.line,
                message: "Unterminated string.".to_string(),
            });
            return;
        }
        // 当探查到 `"` 字符时，结束字符串并调用 advance
        self.advance();
        // let literal = &self.source[self.start + 1..self.current - 1];
        // 字符串（str 类型）是 UTF-8 编码的，因此字符串的底层存储是字节数组
        // 按字符切片，而不是字节切片，因为字符串可能包含非 ASCII 字符
        let literal = &self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - self.start - 2)
            .collect::<String>();
        self.add_token(TokenType::String, Some(String::from(literal)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal = &self.source[self.start..self.current];
        let float = literal
            .parse::<f64>()
            .expect("Number token should be parsed into float");
        let mut value = float.to_string();
        if !value.contains(".") {
            value.push_str(".0");
        }

        self.add_token(TokenType::Number, Some(value));
    }
}
