// Single-character tokens

pub enum TokenType {
    LeftParen,
    RightParen,
    Eof,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::LeftParen => "LEFT_PAREN".to_string(),
            TokenType::RightParen => "RIGHT_PAREN".to_string(),
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
