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
  LParen, RParen, LBrace, RBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Bang, EBang, Equal, EEqual, Gt, Ge,
  Lt, Le, Ident, Str, Num, And, Class, Else, False, For, Fun, If, Nil, Or, Print, Ret, Super, This, True, Var, While,
  Eof
);

impl std::default::Default for TokenType {
  fn default() -> Self {
    TokenType::Eof
  }
}

#[derive(Default, Debug)]
pub struct Token {
  pub typ: TokenType,
  /// the start position of literal in specific source (`Vec<char>`)
  pub start: usize,
  /// the end position (next) of literal in specific source (`Vec<char>`)
  pub end: usize,
  pub line: usize,
}

impl Token {
  /// Format the type, literal and line to readable string.
  pub fn to_string(&self, source: &[char]) -> String {
    format!("[{}: '{}' | {}]", self.typ, self.get_literal(source), self.line)
  }

  /// Retrieve the literal from source
  pub fn get_literal(&self, source: &[char]) -> String {
    // if the toketype is STR, trip the wrapping quote.
    if let TokenType::Str = self.typ {
      unsafe { source.get_unchecked(self.start + 1..self.end - 1).iter().collect() }
    } else {
      unsafe { source.get_unchecked(self.start..self.end).iter().collect() }
    }
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
