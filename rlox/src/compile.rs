use crate::chunk::*;
use crate::scanner::Scanner;
use crate::token::*;

type ParseFn = Option<fn(&mut Compiler)>;
struct ParseRule{
  prefix: ParseFn,
  infix: ParseFn,
  precedence: Precedence,
}

const TOKEN_NUM: usize = 39;

pub struct Compiler<'a>{
  chunk: Chunk,
  current: Token<'a>,
  previous: Token<'a>,
  scanner: Scanner,
  rules: [ParseRule; TOKEN_NUM],
}

impl<'a> Compiler<'a>{
  pub fn new(source: String) -> Self{
    // FIXME This look-up table is extreamely ugly and terrible.
    let rules: [ParseRule; TOKEN_NUM] = [
      ParseRule{prefix: Some(grouping), infix: None, precedence: Precedence::None}, // LParen
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // RParen
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // LBrace
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // RBrace
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Comma
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Dot
      ParseRule{prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term}, // Minus
      ParseRule{prefix: None, infix: Some(binary), precedence: Precedence::Term}, // Plus
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Semicolon
      ParseRule{prefix: None, infix: Some(binary), precedence: Precedence::Factor}, // Slash
      ParseRule{prefix: None, infix: Some(binary), precedence: Precedence::Factor}, // Star
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Bang
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // EBang
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Equal
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // EEqual
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Gt
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Ge
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Lt
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Le
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Ident
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Str
      ParseRule{prefix: Some(number), infix: None, precedence: Precedence::None}, // Num
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // And
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Class
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Else
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // False
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // For
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Fun
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // If
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Nil
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Or
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Print
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Ret
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Super
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // This
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // True
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Var
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // While
      ParseRule{prefix: None, infix: None, precedence: Precedence::None}, // Eof
    ];
    Self{
      chunk: Chunk::new(),
      current: Token::default(),
      previous: Token::default(),
      scanner: Scanner::new(source),
      rules,
    }
  }

  // fn advance(&mut self){
  //   std::mem::swap(&mut self.previous, &mut self.current);
  //   loop {
  //     match self.scanner.scan_token(){
  //       Ok(t) => {
  //         self.current = t;
  //         return;
  //       }
  //       Err(e) => eprintln!("{}", e),
  //     }
  //   }
  // }

}

fn grouping(compiler: &mut Compiler){
  todo!() // TODO
}

fn unary(compiler: &mut Compiler){
  todo!() // TODO
}

fn number(compiler: &mut Compiler){
  todo!() // TODO
}

fn binary(compiler: &mut Compiler){
  todo!() // TODO
}