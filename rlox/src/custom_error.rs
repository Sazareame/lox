#[derive(Debug)]
pub struct ParseError {
  line: usize,
  msg: String,
  literal: String,
}

impl ParseError {
  pub fn new(line: usize, literal: String, msg: String) -> Self {
    Self { line, literal, msg }
  }
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[line {}] Parse Error at {}: {}", self.line, self.literal, self.msg)
  }
}

impl std::error::Error for ParseError {}
