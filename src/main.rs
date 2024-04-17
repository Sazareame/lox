mod scanner;
mod parser;
mod interpreter;

use std::env;
use std::fs;
use std::io::Write;
use crate::scanner::scanner::Scanner;
use crate::parser::parser::Parser;
// use crate::parser::expressions::ast_printer;
use crate::interpreter::evaluate::Interpreter;

struct Runner{
	interpreter: Interpreter
}

impl Runner{
	fn new() -> Self{
		Runner{interpreter: Interpreter::new()}
	}

	fn run_file(&mut self, path: String){
		let source = fs::read_to_string(path).expect("Error during reading source.");
		if let Err(err) = self.run(&source){
			panic!("{}", err);
		}
	}
	
	fn run_prompt(&mut self){
		loop{
			let mut line = String::new();
			std::io::stdout().write_all(b"> ").unwrap();
			std::io::stdout().flush().unwrap();
			std::io::stdin().read_line(&mut line).unwrap();
			if line.is_empty() {break;}
			if let Err(err) = self.run(&line){
				println!("{}", err);
			}
		}
	}
	
	fn run(&mut self, source: &String) -> Result<(), String>{
		let mut scanner = Scanner::new(source);
		let tokens = scanner.scan_tokens()?;
		let mut	parser = Parser::new(tokens);
		let stmts = parser.parse()?;
		// println!("{}", ast_printer(&expressions));
		self.interpreter.interpreter(stmts)?;
		Ok(())
	}
}

fn main(){
	let mut runner = Runner::new();
  match env::args().len(){
    2 => runner.run_file(env::args().nth(1).unwrap()),
    1 => runner.run_prompt(),
    _ => panic!("Usage: rlox [script]"),
  }
}

