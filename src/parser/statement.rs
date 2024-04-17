use crate::parser::expressions::Expr;
use crate::scanner::token ::Token;

pub enum Stmt{
	Print(Box<Expr>),
	Expression(Box<Expr>),
	Block(Vec<Stmt>),
	Var(Token, Box<Expr>),
	None,
}