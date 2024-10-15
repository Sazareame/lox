use crate::chunk::*;
use crate::custom_error::CompileError;
use crate::scanner::Scanner;
use crate::token::*;
use crate::value::Value;

type CompileResult = Result<(), CompileError>;

type ParseFn = Option<fn(&mut Compiler) -> CompileResult>;

struct ParseRule {
  prefix: ParseFn,
  infix: ParseFn,
  precedence: Precedence,
}

const TOKEN_NUM: usize = 39;

pub struct Compiler {
  chunk: Chunk,
  current: Token,
  previous: Token,
  scanner: Scanner,
  rules: [ParseRule; TOKEN_NUM],
}

impl Compiler {
  pub fn new(source: String) -> Self {
    // FIXME This look-up table is extreamely ugly and terrible.
    let rules: [ParseRule; TOKEN_NUM] = [
      ParseRule {
        prefix: Some(grouping),
        infix: None,
        precedence: Precedence::None,
      }, // LParen
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // RParen
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // LBrace
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // RBrace
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Comma
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Dot
      ParseRule {
        prefix: Some(unary),
        infix: Some(binary),
        precedence: Precedence::Term,
      }, // Minus
      ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Term,
      }, // Plus
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Semicolon
      ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
      }, // Slash
      ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
      }, // Star
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Bang
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // EBang
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Equal
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // EEqual
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Gt
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Ge
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Lt
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Le
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Ident
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Str
      ParseRule {
        prefix: Some(number),
        infix: None,
        precedence: Precedence::None,
      }, // Num
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // And
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Class
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Else
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // False
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // For
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Fun
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // If
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Nil
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Or
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Print
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Ret
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Super
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // This
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // True
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Var
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // While
      ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
      }, // Eof
    ];
    Self {
      chunk: Chunk::new(),
      current: Token::default(),
      previous: Token::default(),
      scanner: Scanner::new(source),
      rules,
    }
  }

  fn advance(&mut self) {
    std::mem::swap(&mut self.previous, &mut self.current);
    loop {
      match self.scanner.scan_token() {
        Ok(t) => {
          self.current = t;
          return;
        }
        Err(e) => eprintln!("{}", e),
      }
    }
  }

  fn get_rule(&self, typ: TokenType) -> &ParseRule {
    unsafe { self.rules.get_unchecked(typ as usize) }
  }

  fn expression(&mut self) -> CompileResult {
    self.parse_precedence(Precedence::Assign)
  }

  fn consume(&mut self, typ: TokenType, msg: String) -> CompileResult {
    if self.current.typ == typ {
      self.advance();
      Ok(())
    } else {
      Err(self.raise_at_current(msg))
    }
  }

  /// Raise a `ParseError` from current token.
  fn raise_at_current(&self, msg: String) -> CompileError {
    CompileError::new(self.current.line, self.current.get_literal(self.scanner.source()), msg)
  }

  /// Raise a `ParseError` from previous token.
  fn raise_at_previous(&self, msg: String) -> CompileError {
    CompileError::new(
      self.previous.line,
      self.previous.get_literal(self.scanner.source()),
      msg,
    )
  }

  // Parse the op whose precedence is equal to or higher the `precedence`
  fn parse_precedence(&mut self, precedence: Precedence) -> CompileResult {
    self.advance();
    let prefix_rule = self
      .get_rule(self.previous.typ)
      .prefix
      .ok_or_else(|| self.raise_at_previous("expect expression".into()))?;
    prefix_rule(self)?;
    let mut infix_rule;
    while precedence <= self.get_rule(self.current.typ).precedence {
      self.advance();
      infix_rule = self.get_rule(self.previous.typ).infix.expect("unreachable");
      infix_rule(self)?;
    }
    Ok(())
  }

  /// Do compile, return an `CompileResult` for error handling.
  pub fn compile(&mut self) -> CompileResult {
    self.advance();
    self.expression()?;
    self.consume(TokenType::Eof, "expect end of expression".into())
  }

  /// Emit single bytecode to `self.chunk`
  pub fn emit_byte(&mut self, typ: OpCode) {
    self.chunk.write_chunk(typ, self.previous.line as u8);
  }

  /// Store a constant to constant pool in chunk, then emit a
  /// OP_CONST to chunk.
  pub fn emit_const(&mut self, value: Value) {
    let offset = self.make_const(value);
    self.emit_byte(OpCode::Constant(offset))
  }

  pub fn make_const(&mut self, value: Value) -> u8 {
    self.chunk.write_constant(value)
  }
}

/// Parse grouping expression, emiting bytecode
fn grouping(compiler: &mut Compiler) -> CompileResult {
  compiler.expression()?;
  compiler.consume(TokenType::RParen, "expect ')' after expression".into())
}

/// Parse unary expression, emiting bytecode
fn unary(compiler: &mut Compiler) -> CompileResult {
  use TokenType::*;
  let op_type = compiler.previous.typ;
  compiler.parse_precedence(Precedence::Unary)?;
  match op_type {
    Minus => compiler.emit_byte(OpCode::Neg),
    _ => {}
  }
  Ok(())
}

/// Parse number literal, emiting const bytecode.  
/// This function will panic immediatelly if the char silce `compiler.previous` point to
/// is NOT a meaningful number, which should not happen after correct scanning.
fn number(compiler: &mut Compiler) -> CompileResult {
  let value: f64 = compiler
    .previous
    .get_literal(compiler.scanner.source())
    .parse()
    .expect("Fatal: number literal convert error");
  compiler.emit_const(value);
  Ok(())
}

/// Parse binary expression
fn binary(compiler: &mut Compiler) -> CompileResult {
  use TokenType::*;
  let op_type = compiler.previous.typ;
  let rule = compiler.get_rule(op_type);
  // parse the expresion whose precedence is higher than current op
  // because the binary opration is left associated.
  compiler.parse_precedence(Precedence::higher(&rule.precedence))?;
  match op_type {
    Plus => compiler.emit_byte(OpCode::Add),
    Minus => compiler.emit_byte(OpCode::Sub),
    Star => compiler.emit_byte(OpCode::Mul),
    Slash => compiler.emit_byte(OpCode::Div),
    _ => {}
  }
  Ok(())
}

#[cfg(test)]
mod compile_test {
  use super::*;

  #[test]
  fn test_compile() {
    let source = std::fs::read_to_string("./compile_expression.lox").unwrap();
    let mut compiler = Compiler::new(source);
    compiler.compile().unwrap();
    compiler.chunk.disassembly("result");
    assert!(true);
  }
}
