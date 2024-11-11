
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