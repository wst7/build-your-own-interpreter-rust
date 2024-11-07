use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }
    // 是否到达了文件的结尾
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan_tokens(&mut self) {
      while !self.is_at_end() {
        self.start = self.current;
        self.scan_token();
      }
      self.tokens.push(
        Token::new(TokenType::Eof, "", None, self.line)
      )
    }
    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            _ => {
              self.identifier();
            }
        }
    }
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, None, self.line));
    }
    pub fn identifier(&mut self) {
      
    }
}
