use crate::parser::expressions::Expr;
use crate::parser::statement::Stmt;
use crate::scanner::token::Token;
use crate::scanner::token_type::Object;
use crate::scanner::token_type::TokenType;
use crate::interpreter::environment::Environ;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter{
	environment: Rc<RefCell<Environ>>
}

impl Interpreter{

pub fn new() -> Self{
	Interpreter{environment: Rc::new(RefCell::new(Environ::new(None)))}
}

fn evaluate(&mut self, expr: &Expr) -> Result<Object, String>{
	use Expr::*;
	match expr{
		Literal(value) => Ok(value.clone()),
		Grouping(ptr) => self.evaluate(ptr),
		Unary(operator, right) => {
			let rhs = self.evaluate(right)?;
			match operator.ttype{
				TokenType::MINUS =>	{
					self.check_operand(operator, &rhs)?;
					Ok(-rhs)
				},
				TokenType::BANG => Ok(!rhs),
				_ => Ok(Object::Nil),
			}
		},
		Binary(left, op, right) =>{
			let lhs = self.evaluate(left)?;
			let rhs = self.evaluate(right)?;
			if op.ttype == TokenType::PLUS{
				self.check_addtion(op, &lhs, &rhs)?;
				return Ok(lhs + rhs);
			}
			self.check_operands(op, &lhs, &rhs)?;
			let res = match op.ttype{
				TokenType::MINUS => lhs - rhs,
				TokenType::SLASH => lhs / rhs,
				TokenType::STAR => lhs * rhs,
				TokenType::GREATER => Object::Bool(lhs > rhs),
				TokenType::GREATER_EQUAL => Object::Bool(lhs >= rhs),
				TokenType::LESS => Object::Bool(lhs < rhs),
				TokenType::LESS_EQUAL => Object::Bool(lhs <= rhs),
				TokenType::BANG_EQUAL => Object::Bool(!(lhs == rhs)),
				TokenType::EQUAL_EQUAL => Object::Bool(lhs == rhs),
				_ => Object::Nil,
			};
			Ok(res)
		},
		Variable(name) => self.environment.borrow().get(name),
		Assign(name, value) => {
			let value = self.evaluate(value)?;
			self.environment.borrow_mut().assign(name, &value)?;
			Ok(value)
		}  
		None => Err("None value during evaluate expression.".to_string()),
	}
}

fn execute(&mut self, stmt:  &Stmt) -> Result<Object, String>{
	match stmt{
		Stmt::Expression(expr) => self.evaluate(expr),
		Stmt::Print(expr) => {
			let value = self.evaluate(expr)?;
			println!("{}", value);
			Ok(value)
		},
		Stmt::Var(name, initializer) => {
			let value = if **initializer != Expr::None{
				self.evaluate(initializer)?
			}else{Object::Nil};
			self.environment.borrow_mut().define(name.lexeme.clone(), &value);
			Ok(value)
		},
		Stmt::Block(blocks) => {
			let inner_env = Rc::new(RefCell::new(Environ::new(Some(self.environment.clone()))));
			let previous = self.environment.clone();
			self.environment = inner_env;
			for stmt in blocks{
				if let Err(e) = self.execute(stmt){
					self.environment = previous;
					return Err(e);
				}
			}
			self.environment = previous;
			Ok(Object::Nil)
		}
		Stmt::None => Err("None during execute statement.".to_string()),
	}
}

fn check_operand(&self, op: &Token, oprand: &Object) -> Result<(), String>{
	match oprand{
		Object::Num(_) => Ok(()),
		_ => Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme))
	}
}

fn check_operands(&self, op: &Token, lhs: &Object, rhs: &Object) -> Result<(), String>{
	match lhs{
		Object::Num(_) =>{
			match rhs{
				Object::Num(_) => Ok(()),
				_ => Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme))
			}
		},
		_ => Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme))
	}
}

fn check_addtion(&self, op: &Token, lhs: &Object, rhs: &Object) -> Result<(), String>{
	if let Object::Str(_) = lhs{
		match rhs{
			Object::Str(_) => Ok(()),
			_ => Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme)),
		}
	}else{
		self.check_operands(op, lhs, rhs).or(Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme)))
	}
}

pub fn interpreter(&mut self, stmts: Vec<Stmt>) -> Result<(), String>{
	for stmt in stmts{
		self.execute(&stmt)?;
	}
	Ok(())
}

}

#[cfg(test)]
mod test{
	use super::*;
	use crate::scanner::token_type::{TokenType, Object};

	#[test]
	fn test_print_ast(){
		let expression = Expr::Binary(
			Box::new(Expr::Unary(
				Token::new(TokenType::MINUS, "-".to_string(), Object::Nil, 1),
				Box::new(Expr::Literal(Object::Num(123f64)))
			)),
			Token::new(TokenType::STAR, "*".to_string(), Object::Nil, 1),
			Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Num(45.67)))))
		);

		// assert_eq!(interpreter(&expression), Ok(()));
	}
}