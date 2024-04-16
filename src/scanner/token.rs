use std::fmt::Display;

use crate::scanner::token_type::{TokenType, Object};

#[derive(Clone, PartialEq)]
pub struct Token{
  pub ttype: TokenType,
  pub lexeme: String,
  pub literal: Object,
  pub line: usize,
}

impl Token {
  pub fn new(ttype: TokenType, lexeme: String, literal: Object, line: usize) -> Self{
    Token{ttype, lexeme, literal, line}
  }
}

impl Display for Token{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?} {} {} {}", self.ttype, self.lexeme, self.literal, self.line)
  }
}