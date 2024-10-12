mod chunk;
mod value;
use chunk::*;
fn main() {
  let mut chunk = Chunk::new();
  chunk.write_chunk(OpCode::Return, 1);
  let offset = chunk.write_constant(3.14);
  chunk.write_chunk(OpCode::Constant(offset), 2);
  chunk.write_chunk(OpCode::Return, 3);
  chunk.disassembly("test");
}
