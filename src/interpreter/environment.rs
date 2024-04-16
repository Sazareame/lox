use std::collections::HashMap;
use crate::scanner::{token::Token, token_type::Object};

pub struct Environ{
	values: HashMap<String, Object>
}

impl Environ{
	pub fn new() -> Self{
		Environ{values: HashMap::new()}
	}

	pub fn define(&mut self, name: String, value: &Object){
		self.values.insert(name, value.clone());
	}

	pub fn get(&self, name: &Token) -> Result<Object, String>{
		if let Some(res) = self.values.get(&name.lexeme){
			Ok(res.clone())
		}else{
			Err(format!("line {}: Undefined variable {}", name.line, name.lexeme))
		}
	}
}