use crate::def_tokentype;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Precedence {
  None,
  Assign,
  Or,
  And,
  Equality,
  Comparison,
  Term,
  Factor,
  Unary,
  Call,
  Primary,
}

impl Precedence {
  /// Return the one-level higher precedence over given precedence.
  pub fn higher(prec: &Self) -> Self {
    // FIXME this may have bugs
    match prec {
      Precedence::Primary => panic!("No Such Case"),
      _ => unsafe { std::mem::transmute::<u8, Precedence>(*prec as u8 + 1) },
    }
  }
}

// #[repr(u8)]
// #[derive(Clone, Copy)]
// pub enum TokenType {
def_tokentype!(
  LParen,
  RParen,
  LBrace,
  RBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  EBang,
  Equal,
  EEqual,
  Gt,
  Ge,
  Lt,
  Le,
  Ident,
  Str,
  Num,
  And,
  Class,
  Else,
  False,
  For,
  Fun,
  If,
  Nil,
  Or,
  Print,
  Ret,
  Super,
  This,
  True,
  Var,
  While,
  Eof
);

impl std::default::Default for TokenType{
  fn default() -> Self {
    TokenType::Eof
  }
}

#[derive(Default)]
pub struct Token<'a> {
  pub typ: TokenType,
  pub literal: &'a [char],
  pub line: usize,
}

impl<'a> Token<'a>{
  pub fn to_string(&self) -> String{
    format!("[{}: '{}' | {}]", self.typ, self.literal.iter().collect::<String>(), self.line)
  }
}

// impl<'a> std::default::Default for Token<'a>{
//   fn default() -> Self {
//     Self{
//       typ: TokenType::Eof,
//       literal: &[],
//       line: 0,
//     }
//   }
// }

#[cfg(test)]
mod token_test {
  use super::*;
  #[test]
  fn test_order() {
    assert!(Precedence::None < Precedence::Assign);
    assert!(Precedence::Or <= Precedence::Or);
    assert_eq!(
      unsafe { std::mem::transmute::<u8, Precedence>(Precedence::Term as u8 + 1) },
      Precedence::Factor
    );
    assert_eq!(
      unsafe { std::mem::transmute::<u8, Precedence>(Precedence::Term as u8 + 2) },
      Precedence::Unary
    )
  }
}
