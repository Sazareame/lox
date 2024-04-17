use crate::parser::expressions::Expr::{self, *};
use crate::parser::statement::Stmt;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;
use crate::scanner::token_type::Object;

pub struct Parser{
  tokens: Vec<Token>,
  current: usize,
}

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self{
    Parser{tokens, current: 0}
  }

  fn peek(&self) -> &Token{
    &self.tokens[self.current]
  }

  fn is_at_end(&self) -> bool{
    self.peek().ttype == TokenType::EOF
  }

  fn previous(&self) -> Token{
    self.tokens[self.current - 1].clone()
  }

  fn is_match(&mut self, types: &[TokenType]) -> bool{
    for ttype in types{
      if self.check(ttype){
        self.advance();
        return true;
      }
    }
    false
  }

  fn check(&self, ttype: &TokenType) -> bool{
    if self.is_at_end() {false}
    else{self.peek().ttype == *ttype}
  }

  fn advance(&mut self) -> Token{
    if !self.is_at_end(){
      self.current += 1;
    }
    self.previous()
  }

	fn statement(&mut self) -> Result<Stmt, String>{
		if self.is_match(&[TokenType::PRINT]){
			self.print_statement()
		}else if self.is_match(&[TokenType::LEFT_BRACE]){
			self.block()
		}else{
			self.expression_statement()
		}
	}

	fn block(&mut self) -> Result<Stmt, String>{
		let mut res = Vec::new();
		while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end(){
			res.push(self.declaration());
		}
		self.consume(TokenType::RIGHT_BRACE, "Expect } after block")?;
		Ok(Stmt::Block(res))
	}

	fn declaration(&mut self) -> Stmt{
		let res = if self.is_match(&[TokenType::VAR]){self.var_decl()}else{self.statement()};
		if let Err(e) = res{
			println!("{}", e);
			self.synchronize();
			Stmt::None
		}else{
			res.unwrap()
		}
	}

	fn var_decl(&mut self) -> Result<Stmt, String>{
		let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;
		let initializer = if self.is_match(&[TokenType::EQUAL]){self.expressions()?}else{Expr::None};
		self.consume(TokenType::SEMICOLON, "Expect `;` after variable declaration.")?;
		Ok(Stmt::Var(name, Box::new(initializer)))
	}

	fn print_statement(&mut self) -> Result<Stmt, String>{
		let value = self.expressions()?;
		self.consume(TokenType::SEMICOLON, "Expect `;` after value.")?;
		Ok(Stmt::Print(Box::new(value)))
	}

	fn expression_statement(&mut self) -> Result<Stmt, String>{
		let value = self.expressions()?;
		self.consume(TokenType::SEMICOLON, "Expect `;` after expression.")?;
		Ok(Stmt::Expression(Box::new(value)))
	}

  fn expressions(&mut self) -> Result<Expr, String>{
    self.assignment()
  }

	fn assignment(&mut self) -> Result<Expr, String>{
		let expr = self.equlity()?;
		if self.is_match(&[TokenType::EQUAL]){
			let equals = self.previous();
			let value = self.assignment()?;
			if let Expr::Variable(name) = expr{
				return Ok(Assign(name, Box::new(value)));
			}else{
				return Err(self.error(&equals, "Invalid assignment target."));
			}
		}
		return Ok(expr);
	}

  fn equlity(&mut self) -> Result<Expr, String>{
    let mut expr = self.comparision()?;
    while self.is_match(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]){
      let operator = self.previous();
      let right = self.comparision()?;
      expr = Binary(Box::new(expr), operator, Box::new(right));
    }
		Ok(expr)
  }

  fn comparision(&mut self) -> Result<Expr, String>{
		let mut expr = self.term()?;
		while self.is_match(&[TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]){
			let operator = self.previous();
			let right = self.term()?;
			expr = Binary(Box::new(expr), operator, Box::new(right));
		}
		Ok(expr)
  }

	fn term(&mut self) -> Result<Expr, String>{
		let mut expr = self.factor()?;
		while self.is_match(&[TokenType::MINUS, TokenType::PLUS]){
			let operator = self.previous();
			let right = self.factor()?;
			expr = Binary(Box::new(expr), operator, Box::new(right));
		}
		Ok(expr)
	}

	fn factor(&mut self) -> Result<Expr, String>{
		let mut expr = self.unary()?;
		while self.is_match(&[TokenType::SLASH, TokenType::STAR]){
			let operator = self.previous();
			let right = self.unary()?;
			expr = Binary(Box::new(expr), operator, Box::new(right));
		}
		Ok(expr)
	}

	fn unary(&mut self) -> Result<Expr, String>{
		if self.is_match(&[TokenType::BANG, TokenType::MINUS])	{
			let operator = self.previous();
			let right = self.unary()?;
			return Ok(Unary(operator, Box::new(right)));
		}

		self.primary()
	}

	fn primary(&mut self) -> Result<Expr, String>{
		if self.is_match(&[TokenType::FALSE]){
			return Ok(Literal(Object::Bool(false)));
		}
		if self.is_match(&[TokenType::TRUE]){
			return Ok(Literal(Object::Bool(true)));
		}
		if self.is_match(&[TokenType::NIL]){
			return Ok(Literal(Object::Nil));
		}

		if self.is_match(&[TokenType::STRING, TokenType::NUMBER]){
			return Ok(Literal(self.previous().literal));
		}
		if self.is_match(&[TokenType::LEFT_PAREN]){
			let expr = self.expressions()?;
			self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
			return Ok(Grouping(Box::new(expr)));
		}
		if self.is_match(&[TokenType::IDENTIFIER]){
			return Ok(Variable(self.previous()));
		}
		Err(self.error(self.peek(), "Expect expression"))
	}

	fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<Token, String>{
		if self.check(&ttype){
			Ok(self.advance())
		}else{
			Err(self.error(self.peek(), msg))
		}
	}

	fn error(&self, token: &Token, msg: &str) -> String{
		if token.ttype == TokenType::EOF{
			format!("line {} at the end: {}", token.line, msg)
		}else{
			format!("line {} at '{}': {}", token.line, token.lexeme, msg)
		}
	}

	fn synchronize(&mut self){
		use TokenType::*;
		self.advance();
		while !self.is_at_end(){
			if self.previous().ttype == SEMICOLON{
				return;
			}
			match self.peek().ttype{
				CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
				_ => {}
			}
			self.advance();
		}
	}

	pub fn parse(&mut self) -> Result<Vec<Stmt>, String>{
		let mut stmts = Vec::new();
		while !self.is_at_end(){
			stmts.push(self.declaration());
		}
		Ok(stmts)
	}
}