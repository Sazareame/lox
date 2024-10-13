mod chunk;
mod value;
mod vm;

use chunk::*;
use vm::VM;
fn main() {
  let mut chunk = Chunk::new();
  let offset = chunk.write_constant(3.117);
  chunk.write_chunk(OpCode::Constant(offset), 2);
  chunk.write_chunk(OpCode::Return, 3);
  // chunk.disassembly("test");

  let mut vm = VM::new(chunk);
  vm.run();
}
