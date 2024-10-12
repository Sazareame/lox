use crate::chunk::*;
pub struct VM {
  chunk: Chunk,
  ip: usize,
}

impl VM {
  pub fn new(chunk: Chunk) -> Self {
    Self { chunk, ip: 0 }
  }

  pub fn run(&mut self) {
    use OpCode::*;
    loop {
      match self.chunk.fetch(self.ip) {
        Return => return,
        Constant(val) => println!("{}", self.chunk.get_constant(val.into())),
      }
      self.ip += 1;
    }
  }
}
