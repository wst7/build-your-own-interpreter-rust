

use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    String,
    Number,
    Identifier,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // End of file
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
            TokenType::String => "STRING".to_string(),
            TokenType::Number => "NUMBER".to_string(),
            TokenType::Identifier => "IDENTIFIER".to_string(),
            TokenType::And => "AND".to_string(),
            TokenType::Class => "CLASS".to_string(),
            TokenType::Else => "ELSE".to_string(),
            TokenType::False => "FALSE".to_string(),
            TokenType::Fun => "FUN".to_string(),
            TokenType::For => "FOR".to_string(),
            TokenType::If => "IF".to_string(),
            TokenType::Nil => "NIL".to_string(),
            TokenType::Or => "OR".to_string(),
            TokenType::Print => "PRINT".to_string(),
            TokenType::Return => "RETURN".to_string(),
            TokenType::Super => "SUPER".to_string(),
            TokenType::This => "THIS".to_string(),
            TokenType::True => "TRUE".to_string(),
            TokenType::Var => "VAR".to_string(),
            TokenType::While => "WHILE".to_string(),
            TokenType::Eof => "EOF".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.token_type.to_string(),
            self.lexeme,
            match self.literal {
              Some(ref l) => l,
              None => "null",
          }
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
