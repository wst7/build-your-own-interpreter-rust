use super::token::{Error, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
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

    pub fn scan_tokens(&mut self) -> (&Vec<Token<'a>>, &Vec<Error>) {
      while !self.is_at_end() {
          self.start = self.current;
          self.scan_token();
      }
      self.tokens
          .push(Token::new(TokenType::Eof, "", None, self.line));
      (&self.tokens, &self.errors)
  }

  pub fn get_errors(&self) -> &Vec<Error> {
    &self.errors
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
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '/' => {
                // comment
                if self.next_char_match('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '!' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.next_char_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '"' => self.string(),
            _ => {
                self.errors.push(Error {
                    line: self.line,
                    message: format!("Unexpected character: {}", c),
                });
                // self.identifier();
            }
        }
    }
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }
    // single-character tokens
    pub fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, None, self.line));
    }
    fn add_token_literal(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        let literal = &self.source[self.start + 1..self.current - 1];
        self.tokens
            .push(Token::new(token_type, text, Some(literal), self.line));
      
    }
    fn identifier(&mut self) {}

    
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
      // peek 探查到下一个字符是 "
      self.advance();
      self.add_token_literal(TokenType::String);
    }
}
