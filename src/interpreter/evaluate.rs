use crate::parser::expressions::Expr;
use crate::parser::statement::Stmt;
use crate::scanner::callable::BuiltinFunc;
use crate::scanner::token::Token;
use crate::scanner::token_type::Object;
use crate::scanner::callable::FuncType;
use crate::scanner::token_type::TokenType;
use crate::interpreter::environment::Environ;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum RuntimeMsg{
	Err(String),
	Ret(Object),
}

pub struct Interpreter{
	pub environment: Rc<RefCell<Environ>>,
}

impl Interpreter{

pub fn new() -> Self{
	let environment = Rc::new(RefCell::new(Environ::new(None)));
	fn clock(_: &mut Interpreter, _: Vec::<Object>) -> Result<Object, RuntimeMsg>{
		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
		Ok(Object::Num(now))
	}
	let builtin_clock_func = Object::Callabe(Rc::new(BuiltinFunc::new(clock, 0)));
	environment.borrow_mut().define("clock".to_string(), builtin_clock_func);
	Interpreter{environment}
	// Interpreter{environment: Rc::new(RefCell::new(Environ::new(None)))}
}

fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeMsg>{
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
		Variable(name) => self.environment.borrow().get(name).map_err(|e| RuntimeMsg::Err(e)),
		Assign(name, value) => {
			let value = self.evaluate(value)?;
			self.environment.borrow_mut().assign(name, value.clone()).map_err(|e| RuntimeMsg::Err(e))?;
			Ok(value)
		},
		Logical(lhs, operator, rhs) => {
			let left = self.evaluate(lhs)?;
			if operator.ttype == TokenType::OR{
				if left.is_true(){
					return Ok(left);
				}
			}else if !left.is_true(){
					return Ok(left);
			}
			self.evaluate(rhs)
		},
		Call(callee, _, args) => {
			let callee = self.evaluate(callee)?;
			// The callee could be either an Identifier or another call expression
			// change the lookup table into HashMap<String, E>, where E can be Object or Callabe type
			let mut arguments = Vec::new();
			for arg in args{
				arguments.push(self.evaluate(arg)?);
			}
			if let Object::Callabe(func) = callee{
				if arguments.len() != func.arity(){
					return Err(RuntimeMsg::Err(format!("In function: {}, expected {} arguments, got {}.", func, func.arity(), arguments.len())));
				}
				let res = func.call(self, arguments)?;
				return Ok(res);
			}else{
				return Err(RuntimeMsg::Err(format!("type {} is not callable type", callee)));
			}
		},
		None => Err(RuntimeMsg::Err("None value during evaluating expression.".to_string())),
	}
}

pub fn execute(&mut self, stmt:  &Stmt) -> Result<(), RuntimeMsg>{
	match stmt{
		Stmt::Expression(expr) => {self.evaluate(expr)?;}
		Stmt::Print(expr) => {
			let value = self.evaluate(expr)?;
			println!("{}", value);
		},
		Stmt::Var(name, initializer) => {
			let value = if **initializer != Expr::None{
				self.evaluate(initializer)?
			}else{Object::Nil};
			self.environment.borrow_mut().define(name.lexeme.clone(), value);
		},
		Stmt::Block(blocks) => {
			let previous = self.environment.clone();
			let inner_env = Rc::new(RefCell::new(Environ::new(Some(self.environment.clone()))));
			self.environment = inner_env;
			for stmt in blocks{
				if let Err(e) = self.execute(stmt){
					self.environment = previous;
					return Err(e);
				}
			}
			self.environment = previous;
		},
		Stmt::IfStmt(condition, then, els) =>{
			let eval_res = self.evaluate(condition)?;
			if let Object::Bool(false) = !eval_res{
				return self.execute(then);
			}
			if let Stmt::None = **els{
				return Ok(())
			}
			self.execute(els)?;
		},
		Stmt::WhileStmt(condition, body) => {
			while self.evaluate(condition)?.is_true(){
				self.execute(body)?;
			}
		},
		Stmt::Function(name, _, _) => {
			self.environment.borrow_mut().define(name.lexeme.clone(), Object::Callabe(Rc::new(FuncType::new(&stmt))));
		},
		Stmt::ReturnStmt(_, value) => {
			let ret = if let Expr::None = value.as_ref(){
				Object::Nil
			}else{
				self.evaluate(value)?
			};
			return Err(RuntimeMsg::Ret(ret));
		},
		Stmt::None => {return Err(RuntimeMsg::Err("None during execute statement.".to_string()));}
	}
	Ok(())
}

fn check_operand(&self, op: &Token, oprand: &Object) -> Result<(), RuntimeMsg>{
	match oprand{
		Object::Num(_) => Ok(()),
		_ => Err(RuntimeMsg::Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme)))
	}
}

fn check_operands(&self, op: &Token, lhs: &Object, rhs: &Object) -> Result<(), RuntimeMsg>{
	match lhs{
		Object::Num(_) =>{
			match rhs{
				Object::Num(_) => Ok(()),
				_ => Err(RuntimeMsg::Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme))),
			}
		},
		_ => Err(RuntimeMsg::Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme)))
	}
}

fn check_addtion(&self, op: &Token, lhs: &Object, rhs: &Object) -> Result<(), RuntimeMsg>{
	if let Object::Str(_) = lhs{
		match rhs{
			Object::Str(_) => Ok(()),
			_ => Err(RuntimeMsg::Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme))),
		}
	}else{
		self.check_operands(op, lhs, rhs).or(Err(RuntimeMsg::Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme))))
	}
}

pub fn interpreter(&mut self, stmts: Vec<Stmt>) -> Result<(), String>{
	for stmt in stmts{
		if let Err(RuntimeMsg::Err(e)) = self.execute(&stmt){
			return Err(e);
		}
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