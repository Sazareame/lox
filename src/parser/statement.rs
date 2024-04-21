use crate::parser::expressions::Expr;
use crate::scanner::token ::Token;

#[derive(Clone)]
pub enum Stmt{
	Print(Box<Expr>),
	Expression(Box<Expr>),
	Function(Token, Vec<Token>, Box<Stmt>),
	Block(Vec<Stmt>),
	Var(Token, Box<Expr>),
	IfStmt(Box<Expr>, Box<Stmt>, Box<Stmt>),
	WhileStmt(Box<Expr>, Box<Stmt>),
	ReturnStmt(Token, Box<Expr>),
	None,
}