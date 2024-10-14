use crate::custom_error::ParseError;
use crate::token::*;

pub struct Scanner {
  start: usize,
  current: usize,
  line: usize,
  source: Vec<char>,
}

type ScanResult = Result<Token, ParseError>;

// Constructor
impl Scanner {
  /// Construct a scanner from a `String`, then append a `'\0'`.
  pub fn new(source: String) -> Self {
    let mut source: Vec<char> = source.chars().collect();
    source.push('\0');
    Self {
      start: 0,
      current: 0,
      line: 1,
      source,
    }
  }
}

// help functions
impl Scanner {
  fn get(&self, index: usize) -> char {
    unsafe { *self.source.get_unchecked(index) }
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.get(self.current - 1)
  }

  fn peek(&self) -> char {
    self.get(self.current)
  }

  fn skip_whitespace(&mut self){
    loop{
      match self.peek(){
        c if !c.is_ascii_whitespace() => return,
        '\n' => self.line += 1,
        '/' => {
          if self.is_match('/'){
            while self.peek() != '\n' && !self.is_at_end(){
              self.advance();
            }
          }else{
            return;
          }
        }
        _ => {}
      }
      self.advance();
    }
  }

  fn peek_next(&self) -> Option<char> {
    if self.is_at_end() {
      None
    } else {
      Some(self.get(self.current + 1))
    }
  }

  /// Check whether the next character is the given `expected`, if true, consume that character.
  fn is_match(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }
    if self.peek() != expected {
      return false;
    }
    self.current += 1;
    true
  }

  fn is_at_end(&self) -> bool {
    self.peek() == '\0'
  }

  fn is_legal_ident(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
  }

  fn scan_number(&mut self) -> ScanResult {
    while self.peek().is_ascii_digit() {
      self.advance();
    }
    if self.is_match('.') && self.peek().is_ascii_digit() {
      while self.peek().is_ascii_digit() {
        self.advance();
      }
    }
    self.make_token(TokenType::Num)
  }

  fn scan_string(&mut self) -> ScanResult {
    while self.peek() != '"' && !self.is_at_end() {
      self.advance();
    }
    if self.is_match('"') {
      self.make_token(TokenType::Str)
    } else {
      Err(ParseError::new(
        self.line,
        self.get(self.current - 1).into(),
        "unterminated string".into(),
      ))
    }
  }

  fn scan_ident(&mut self) -> ScanResult{
    while Self::is_legal_ident(self.peek()) || self.peek().is_ascii_digit(){
      self.advance();
    }
    let typ = self.scan_ident_type();
    self.make_token(typ)
  }

  fn scan_ident_type(&mut self) -> TokenType {
    use TokenType::*;
    match self.get(self.start) {
      'a' => self.check_keyword(1, 2, "nd", And),
      'c' => self.check_keyword(1, 4, "lass", Class),
      'e' => self.check_keyword(1, 3, "lse", Else),
      'i' => self.check_keyword(1, 1, "f", If),
      'n' => self.check_keyword(1, 2, "il", Nil),
      'o' => self.check_keyword(1, 1, "r", Or),
      'p' => self.check_keyword(1, 4, "rint", Print),
      'r' => self.check_keyword(1, 5, "eturn", Ret),
      's' => self.check_keyword(1, 4, "uper", Super),
      'v' => self.check_keyword(1, 2, "ar", Var),
      'w' => self.check_keyword(1, 4, "hlie", While),
      'f' if self.current - self.start > 1 => match self.get(self.start + 1) {
        'a' => self.check_keyword(2, 3, "lse", False),
        'o' => self.check_keyword(2, 1, "r", For),
        'u' => self.check_keyword(2, 1, "n", Fun),
        _ => Ident,
      },
      't' if self.current - self.start > 1 => match self.get(self.start + 1) {
        'h' => self.check_keyword(2, 2, "is", This),
        'r' => self.check_keyword(2, 2, "ue", True),
        _ => Ident,
      },
      _ => Ident,
    }
  }

  fn check_keyword(&self, start: usize, len: usize, rest: &str, typ: TokenType) -> TokenType {
    let cmp_start = self.start + start;
    let cmp_end = cmp_start + len;
    if self.current - self.start == start + len &&
    unsafe{self.source.get_unchecked(cmp_start..cmp_end).iter().zip(rest.chars()).all(|(c1, c2)| *c1 == c2)}{
      typ
    }else{
      TokenType::Ident
    }
  }

  fn make_token(&self, typ: TokenType) -> ScanResult {
    Ok(Token {
      typ,
      start: self.start,
      end: self.current,
      line: self.line,
    })
  }

  /// Make a token with further check. If the next character matches `expected`, make token with `typ1`, else `typ2`.
  fn make_token_with_check(&mut self, typ1: TokenType, typ2: TokenType, expected: char) -> ScanResult {
    if self.is_match(expected) {
      self.make_token(typ1)
    } else {
      self.make_token(typ2)
    }
  }
}

impl Scanner {
  pub fn scan_token(&mut self) -> ScanResult {
    use TokenType::*;

    self.skip_whitespace();

    self.start = self.current;
    if self.is_at_end() {
      return self.make_token(Eof);
    }

    let ch = self.advance();
    match ch {
      '(' => self.make_token(LParen),
      ')' => self.make_token(RParen),
      '{' => self.make_token(LBrace),
      '}' => self.make_token(RBrace),
      ';' => self.make_token(Semicolon),
      ',' => self.make_token(Comma),
      '.' => self.make_token(Minus),
      '+' => self.make_token(Plus),
      '/' => self.make_token(Slash),
      '*' => self.make_token(Star),
      '!' => self.make_token_with_check(EBang, Bang, '='),
      '=' => self.make_token_with_check(EEqual, Equal, '='),
      '<' => self.make_token_with_check(Le, Lt, '='),
      '>' => self.make_token_with_check(Ge, Gt, '='),
      '"' => self.scan_string(),
      c if c.is_ascii_digit() => self.scan_number(),
      c if Self::is_legal_ident(c) => self.scan_ident(),
      _ => Err(ParseError::new(self.line, ch.into(), "unexpect character".into())),
    }
  }
}

#[cfg(test)]
mod scanner_test {
  use super::*;

  #[test]
  fn test_scanner() {
    let source = std::fs::read_to_string("./test.lox").unwrap();
    let mut scanner = Scanner::new(source);
    loop{
      match scanner.scan_token(){
        Ok(t) => {
          println!("{}", t.to_string(&scanner.source));
          if t.typ == TokenType::Eof{
            break;
          }
        }
        Err(e) => eprintln!("{}", e),
      }
    } 
  }
}
