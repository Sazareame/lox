mod chunk;
mod value;
use chunk::*;
fn main() {
  let mut chunk = Chunk::new();
  chunk.write_chunk(OpCode::Return);
  let offset = chunk.write_constant(3.14);
  chunk.write_chunk(OpCode::Constant(offset));
  chunk.write_chunk(OpCode::Return);
  chunk.disassembly("test");
}
