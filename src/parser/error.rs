
#[derive(Debug)]
pub struct ParseError {
  message: String,
  line: usize,
}

impl ParseError {
  pub fn new(message: &str, line: usize) -> Self {
    Self {
      message: message.to_string(),
      line,
    }
  }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}