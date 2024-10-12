mod chunk;
use chunk::*;
fn main() {
  let mut chunk = Chunk::new();
  chunk.write_chunk(OpCode::Return);
  chunk.disassembley("test");
}
