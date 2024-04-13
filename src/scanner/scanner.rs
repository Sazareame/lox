use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType::{self, *};
use super::token_type::Object;
use std::collections::HashMap;

pub struct Scanner <'a>{
  source: &'a str,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
  key_words: HashMap<&'a str, TokenType>
}

impl <'a>Scanner<'a>{
  pub fn new(source: &'a str) -> Self{
    let key_words = HashMap::from([
      ("and", AND),
      ("class", CLASS),
      ("else",   ELSE),
      ("false",  FALSE),
      ("for",    FOR),
      ("fun",    FUN),
      ("if",     IF),
      ("nil",    NIL),
      ("or",     OR),
      ("print",  PRINT),
      ("return", RETURN),
      ("super",  SUPER),
      ("this",   THIS),
      ("true",   TRUE),
      ("var",    VAR),
      ("while",  WHILE),
    ]);

    Scanner{
      source,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      key_words}
  }

  fn is_at_end(&self) -> bool{
    self.current >= self.source.len() 
  }

  fn advance(&mut self) -> char{
    let c = self.source.chars().nth(self.current).unwrap();
    self.current += 1;
    c
  }

  fn add_token_null(&mut self, ttype: TokenType){
    self.add_token(ttype, Object::Nil);
  }

  fn add_token(&mut self, ttype: TokenType, literal: Object){
    let text = self.source[self.start..self.current].to_string();
    self.tokens.push(Token::new(ttype, text, literal, self.line))
  }

  fn is_match(&mut self, expected: char) -> bool{
    if self.is_at_end() {return false;}
    if self.source.chars().nth(self.current).unwrap() != expected {return false;}
    self.current += 1;
    true
  }

  fn peek(&self) -> char{
    if self.is_at_end() {return '\0';}
    self.source.chars().nth(self.current).unwrap()
  }

  fn peek_next(&self) -> char{
    if self.current + 1 >= self.source.len() {'\0'}
    else{self.source.chars().nth(self.current + 1).unwrap()}
  }

  fn string(&mut self) -> Result<(), String>{
    while self.peek() != '"' && !self.is_at_end(){
      if self.peek() == '\n' {self.line += 1;}
      self.advance();
    }
    if self.is_at_end() {
      return Err(format!("line {}: unterminated string.", self.line));
    }
    self.advance();
    let s = &self.source[self.start + 1..self.current - 1];
    self.add_token(STRING, Object::Str(s.to_string()));
    Ok(())
  }

  fn number(&mut self){
    while self.peek().is_ascii_digit() {self.advance();}
    if self.peek() == '.' && self.peek_next().is_ascii_digit(){
      self.advance();
      while self.peek().is_ascii_digit() {self.advance();}
    } 
    self.add_token(
      NUMBER,
      Object::Num(self.source[self.start..self.current].parse().unwrap()))
  }

  fn identifier(&mut self){
    while self.peek().is_ascii_alphanumeric() {self.advance();}
    let text = &self.source[self.start..self.current];
    if let Some(t) = self.key_words.get(text){
      self.add_token_null(t.clone());
    }else{
      self.add_token_null(IDENTIFIER);
    }
  }

  pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String>{
    while !self.is_at_end(){
      self.start = self.current;
      self.scan_token()?;
    }  
    self.tokens.push(Token::new(
      EOF, " ".to_string(), Object::Nil, self.line));
    Ok(self.tokens.clone())
  }

  fn scan_token(&mut self) -> Result<(), String>{
    let c = self.advance();
    match c {
      '(' => self.add_token_null(LEFT_PAREN),
      ')' => self.add_token_null(RIGHT_PAREN),
      '{' => self.add_token_null(LEFT_BRACE),
      '}' => self.add_token_null(RIGHT_BRACE),
      ',' => self.add_token_null(COMMA),
      '.' => self.add_token_null(DOT),
      '-' => self.add_token_null(MINUS),
      '+' => self.add_token_null(PLUS),
      ';' => self.add_token_null(SEMICOLON),
      '*' => self.add_token_null(STAR),
      '!' => {
        if self.is_match('='){self.add_token_null(BANG_EQUAL)}
        else{self.add_token_null(BANG)}
      }
      '=' => {
        if self.is_match('='){self.add_token_null(EQUAL_EQUAL)}
        else{self.add_token_null(EQUAL)}
      }
      '<' => {
        if self.is_match('='){self.add_token_null(LESS_EQUAL)}
        else{self.add_token_null(LESS)}
      }
      '>' => {
        if self.is_match('='){self.add_token_null(GREATER_EQUAL)}
        else{self.add_token_null(GREATER)}
      }
      '/' => {
        if self.is_match('/'){
          while self.peek() != '\n' && !self.is_at_end() {self.advance();}
        }else{
          self.add_token_null(SLASH);
        }
      }
      ' ' | '\r' | '\t' => {}
      '\n' => self.line += 1,
      '"' => self.string()?,
      '0'..='9' => self.number(),
      'a'..='z' | 'A'..='Z' => self.identifier(),
      _ => {
        let s = format!("[line {}] Error: unexpected character.", self.line);
        return Err(s);
      }
    }
    Ok(())
  }
}