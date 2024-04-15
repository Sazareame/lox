use crate::parser::expressions::Expr;
use crate::scanner::token::Token;
use crate::scanner::token_type::Object;
use crate::scanner::token_type::TokenType;

fn evaluate(expr: &Expr) -> Result<Object, String>{
	use Expr::*;
	match expr{
		Literal(value) => Ok(value.clone()),
		Grouping(ptr) => evaluate(ptr),
		Unary(operator, right) => {
			let rhs = evaluate(right)?;
			match operator.ttype{
				TokenType::MINUS =>	{
					check_operand(operator, &rhs)?;
					Ok(-rhs)
				},
				TokenType::BANG => Ok(!rhs),
				_ => Ok(Object::Nil),
			}
		},
		Binary(left, op, right) =>{
			let lhs = evaluate(left)?;
			let rhs = evaluate(right)?;
			if op.ttype == TokenType::PLUS{
				check_addtion(op, &lhs, &rhs)?;
				return Ok(lhs + rhs);
			}
			check_operands(op, &lhs, &rhs)?;
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
		}
	}
}

fn check_operand(op: &Token, oprand: &Object) -> Result<(), String>{
	match oprand{
		Object::Num(_) => Ok(()),
		_ => Err(format!("line {}: operand of {} must be number.", op.line, op.lexeme))
	}
}

fn check_operands(op: &Token, lhs: &Object, rhs: &Object) -> Result<(), String>{
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

fn check_addtion(op: &Token, lhs: &Object, rhs: &Object) -> Result<(), String>{
	if let Object::Str(_) = lhs{
		match rhs{
			Object::Str(_) => Ok(()),
			_ => Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme)),
		}
	}else{
		check_operands(op, lhs, rhs).or(Err(format!("line {}: operand of {} must be number or string.", op.line, op.lexeme)))
	}
}

pub fn interpreter(expr: &Expr) -> Result<(), String>{
	let value = evaluate(expr)?;
	println!("{}", value.to_string());
	Ok(())
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

		assert_eq!(interpreter(&expression), Ok(()));
	}
}