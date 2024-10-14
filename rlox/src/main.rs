#![allow(dead_code)]
mod chunk;
mod value;
mod vm;
#[macro_use]
mod def_macro;
mod custom_error;
mod scanner;
mod token;
mod compile;

use chunk::*;
use vm::VM;
fn main() {
  let mut chunk = Chunk::new();

  let mut offset = chunk.write_constant(1.2);
  chunk.write_chunk(OpCode::Constant(offset), 2);

  offset = chunk.write_constant(3.4);
  chunk.write_chunk(OpCode::Constant(offset), 2);

  chunk.write_chunk(OpCode::Add, 2);

  offset = chunk.write_constant(5.6);
  chunk.write_chunk(OpCode::Constant(offset), 2);

  chunk.write_chunk(OpCode::Div, 2);
  chunk.write_chunk(OpCode::Neg, 2);
  chunk.write_chunk(OpCode::Return, 3);

  let mut vm = VM::new(chunk);
  vm.run();
}
