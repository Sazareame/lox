use std::fmt::Display;
use std::iter::zip;
use std::{cell::RefCell, rc::Rc};
use crate::parser::statement::Stmt;
use crate::interpreter::environment::Environ;
use crate::scanner::token_type::Object;
use crate::interpreter::evaluate::Interpreter;
use crate::interpreter::evaluate::RuntimeMsg;

pub trait Callable: Display{
	fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, RuntimeMsg>;
	fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct FuncType{
	declaration: Box<Stmt>,
	arity: usize,
	closure: Rc<RefCell<Environ>>,
}

impl FuncType{
	pub fn new(definition: &Stmt, closure_env: &Rc<RefCell<Environ>>) -> Self{
		if let Stmt::Function(_, params, _) = definition{
			return Self{declaration: Box::new(definition.clone()), arity: params.len(), closure: closure_env.clone()}
		}
		panic!("It should execute into such circumstance, and that is why I donot like rust's Enum.");
	}
}

impl Callable for FuncType{
	fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, RuntimeMsg>{
		let call_env = Rc::new(RefCell::new(Environ::new(Some(self.closure.clone()))));
		if let Stmt::Function(_, params, body) = self.declaration.as_ref(){
			for (param, arg) in zip(params, args){
				call_env.borrow_mut().define(param.lexeme.clone(), arg);
			}
			let previous = interpreter.environment.clone();
			interpreter.environment = call_env;
			let res = match interpreter.execute(body){
				Err(RuntimeMsg::Err(e)) => Err(RuntimeMsg::Err(e)),
				Err(RuntimeMsg::Ret(ret)) => Ok(ret),
				_ => Ok(Object::Nil),
			};
			interpreter.environment = previous;
			return res;
		}
		panic!("It should execute into such circumstance, and that is why I donot like rust's Enum.");
	}

	fn arity(&self) -> usize{
		self.arity
	}
}

impl Display for FuncType{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Stmt::Function(name, _, _) = self.declaration.as_ref(){
			write!(f, "<function object {}>", name.lexeme.clone())
		}else{
			panic!("It should execute into such circumstance, and that is why I donot like rust's Enum.");
		}
	}
}

pub struct BuiltinFunc{
	body: fn(&mut Interpreter, Vec<Object>) -> Result<Object, RuntimeMsg>,
	arity: usize,
}

impl Display for BuiltinFunc{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<builtin function>")
	}
}

impl BuiltinFunc{
	pub fn new(body: fn(&mut Interpreter, Vec<Object>) -> Result<Object, RuntimeMsg>, arity: usize) -> Self{
		Self{body, arity}
	}
}

impl Callable for BuiltinFunc{
	fn arity(&self) -> usize {
		self.arity
	}

	fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, RuntimeMsg> {
		(self.body)(interpreter, args)
	}
}