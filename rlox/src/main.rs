#![allow(dead_code)]
mod chunk;
mod value;
mod vm;
#[macro_use]
mod def_macro;
mod compile;
mod custom_error;
mod scanner;
mod token;

use crate::vm::VM;
use crate::compile::Compiler;

fn parse_args() -> Option<String> {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() > 2 {
    eprintln!("Unexpected argument(s)!. Usage: lox [script]");
    std::process::exit(1);
  }
  assert!(!args.is_empty());
  if args.len() == 1 {
    None
  }else{
    Some(args[1].clone())
  }
}

fn repl() {
  use std::io::BufRead;
  use std::io::Write;
  let mut reader = std::io::BufReader::new(std::io::stdin());
  let mut buf = String::new();
  loop {
    print!("> ");
    std::io::stdout().flush().unwrap();
    reader.read_line(&mut buf).unwrap();
    if buf.is_empty(){
      continue;
    } 
    let mut compiler = Compiler::new(buf.clone());
    if let Err(e) = compiler.compile(){
      eprintln!("{}", e);
      continue;
    }
    let mut vm = VM::new(compiler.return_chunk());
    vm.run();
    buf.clear();
  }
}

fn run_source(path: String) {
  let source = std::fs::read_to_string(path);
  if let Err(e) = source{
    eprintln!("Error during read file: {}", e);
    std::process::exit(1);
  }else{
    let mut compiler = Compiler::new(source.unwrap());
    if let Err(e) = compiler.compile(){
      eprintln!("{}", e);
      std::process::exit(1);
    }
    let mut vm = VM::new(compiler.return_chunk());
    vm.run();
  }
}

fn main() {
  let path = parse_args();
  if let Some(path) = path{
    run_source(path);
  }else{
    repl();
  }
}
