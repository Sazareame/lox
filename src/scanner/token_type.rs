use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum TokenType{
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  IDENTIFIER, STRING, NUMBER,

  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF,
}

#[derive(Clone)]
pub enum Object{
  Num(f64),
  Str(String),
  Nil,
  Bool(bool),
}

impl Display for Object{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self{
      Object::Num(n) => write!(f, "{}", n),
      Object::Bool(b) => write!(f, "{}", b),
      Object::Nil => write!(f, "Nil"),
      Object::Str(s) => write!(f, "{}", s),
    }   
  }
}