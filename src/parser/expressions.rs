use crate::scanner::token::Token;
use crate::scanner::token_type::Object;

#[derive(PartialEq)]
pub enum Expr{
	Binary(Box<Expr>, Token, Box<Expr>),
	Grouping(Box<Expr>),
	Literal(Object),
	Unary(Token, Box<Expr>),
	Variable(Token),
	None,
}

pub fn ast_printer(expr: &Expr) -> String{
	use Expr::*;
	match expr{
		Binary(left, operator, right) => parenthesize(&operator.lexeme, &[left, right]),
		Grouping(expression) => parenthesize("group", &[expression]),
		Literal(value) => value.to_string(),
		Unary(operator, right) => parenthesize(&operator.lexeme, &[right]),
		Variable(name) => name.to_string(),
		None => "None".to_string(),
	}
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String{
	let mut s = String::from("(");
	s.push_str(name);
	for expr in exprs{
		s.push(' ');
		s.push_str(&ast_printer(expr))
	}
	s.push(')');
	s
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

		let s = ast_printer(&expression);
		assert_eq!(s, String::from("(* (- 123) (group 45.67))"));

	}
}