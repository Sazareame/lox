use std::collections::HashMap;
use crate::scanner::{token::Token, token_type::Object};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environ{
	values: HashMap<String, Object>,
	enclosing: Option<Rc<RefCell<Environ>>>,
}

impl Environ{
	pub fn new(enclosing: Option<Rc<RefCell<Environ>>>) -> Self{
			Environ{values: HashMap::new(), enclosing}
	}

	pub fn define(&mut self, name: String, value: Object){
		self.values.insert(name, value);
	}

	pub fn get(&self, name: &Token) -> Result<Object, String>{
		if let Some(res) = self.values.get(&name.lexeme){
			Ok(res.clone())
		}else{
			if self.enclosing.is_some(){
				return self.enclosing.clone().unwrap().borrow().get(name);
			}
			Err(format!("line {}: Undefined variable {}", name.line, name.lexeme))
		}
	}

	pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), String>{
		if let Some(_) = self.values.get(&name.lexeme){
			self.values.insert(name.lexeme.clone(), value);
			Ok(())
		}else{
			if self.enclosing.is_some(){
				return self.enclosing.clone().unwrap().borrow_mut().assign(name, value);
			}
			Err(format!("line {}: Undefined variable {}", name.line, name.lexeme))
		}
	}
}