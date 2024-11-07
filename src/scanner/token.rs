

use std::fmt::{self, Display};

pub enum TokenType {
  // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Eof,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::LeftParen => "LEFT_PAREN".to_string(),
            TokenType::RightParen => "RIGHT_PAREN".to_string(),
            TokenType::LeftBrace => "LEFT_BRACE".to_string(),
            TokenType::RightBrace => "RIGHT_BRACE".to_string(),
            TokenType::Comma => "COMMA".to_string(),
            TokenType::Dot => "DOT".to_string(),
            TokenType::Minus => "MINUS".to_string(),
            TokenType::Plus => "PLUS".to_string(),
            TokenType::Semicolon => "SEMICOLON".to_string(),
            TokenType::Star => "STAR".to_string(),
            TokenType::Slash => "SLASH".to_string(),
            TokenType::Bang => "BANG".to_string(),
            TokenType::BangEqual => "BANG_EQUAL".to_string(),
            TokenType::Equal => "EQUAL".to_string(),
            TokenType::EqualEqual => "EQUAL_EQUAL".to_string(),
            TokenType::Greater => "GREATER".to_string(),
            TokenType::GreaterEqual => "GREATER_EQUAL".to_string(),
            TokenType::Less => "LESS".to_string(),
            TokenType::LessEqual => "LESS_EQUAL".to_string(),
            TokenType::Eof => "EOF".to_string(),
        }
    }
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<&'a str>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<&'a str>,
        line: usize,
    ) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token<'_> {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.token_type.to_string(),
            self.lexeme,
            self.literal.map_or("null", |x| x)
        )
    }
}

pub struct Error {
    pub message: String,
    pub line: usize,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[line {}] Error: {}", self.line, self.message)
    }
}
