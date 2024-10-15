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

use clap::{Arg, ArgAction, Command};

struct Args {
  print_scan_res: bool,
  print_bytecode_res: bool,
  input: Option<String>,
}

fn parse_args() -> Args {
  let args = Command::new("rlox")
    .author("Sazareame")
    .about("Lox Interpreter in Rust")
    .version("0.1.0")
    .arg(Arg::new("input").help("Input source file"))
    .arg(Arg::new("lexer").short('l').long("lex").action(ArgAction::SetTrue))
    .arg(Arg::new("code").short('c').long("code").action(ArgAction::SetTrue))
    .get_matches();

  Args {
    print_scan_res: args.get_flag("lexer"),
    print_bytecode_res: args.get_flag("code"),
    input: args.get_one::<String>("input").cloned(),
  }
}

fn main() {}
