#[derive(Debug)]
pub struct CompileError {
  line: usize,
  msg: String,
  literal: String,
}

impl CompileError {
  pub fn new(line: usize, literal: String, msg: String) -> Self {
    Self { line, literal, msg }
  }
}

impl std::fmt::Display for CompileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[line {}] Error Error at {}: {}", self.line, self.literal, self.msg)
  }
}

impl std::error::Error for CompileError {}
