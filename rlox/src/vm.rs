use crate::chunk::*;
use crate::value::Value;

const MAX_STACK: usize = 255;
pub struct VM {
  chunk: Chunk,
  stack: [Value; MAX_STACK],
  /// Aka. `%rip`, which points to the **next** instruction.
  ip: usize,
  /// Aka. `%rsp`, which points to the **next** postion on stack.
  sp: usize,
}

impl VM {
  pub fn new(chunk: Chunk) -> Self {
    Self { 
      chunk, 
      stack: [f64::default(); MAX_STACK],
      ip: 0,
      sp: 0,
    }
  }

  /// Push `value` to stack
  fn push(&mut self, value: Value) {
    unsafe{*self.stack.get_unchecked_mut(self.sp) = value;}
    self.sp += 1;
  }

  fn pop(&mut self) -> Value {
    self.sp -= 1;
    unsafe{std::mem::take(self.stack.get_unchecked_mut(self.sp))}
  }

  pub fn run(&mut self) {
    use OpCode::*;
    let mut ins;
    loop {
      ins = self.chunk.fetch(self.ip);

      println!("{}", ins);
      println!("== stack ==");
      for i in 0..self.sp{
        print!("[ {} ]", unsafe {
          self.stack.get_unchecked(i)
        })
      }
      println!();

      match ins {
        Return => {
          println!("{}\n", self.pop());
          return;
        }
        Constant(val) => {
          let constant = self.chunk.get_constant(val.into());
          self.push(*constant);
        }
      }
      self.ip += 1;
    }
  }
}
