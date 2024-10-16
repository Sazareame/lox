use crate::chunk::*;
use crate::custom_error;
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

macro_rules! binary{
  ($vm:ident, $op: tt) => {
    {
    let rhs = $vm.pop();
    let lhs = $vm.pop();
    $vm.push(lhs $op rhs);
    }
  }
}

impl VM {
  pub fn new(chunk: Chunk) -> Self {
    Self {
      chunk,
      stack: [Value::default(); MAX_STACK],
      ip: 0,
      sp: 0,
    }
  }

  /// Push `value` to stack
  fn push(&mut self, value: Value) {
    unsafe {
      *self.stack.get_unchecked_mut(self.sp) = value;
    }
    self.sp += 1;
  }

  fn pop(&mut self) -> Value {
    self.sp -= 1;
    unsafe { std::mem::take(self.stack.get_unchecked_mut(self.sp)) }
  }

  /// Peek the stack value with `offset` from the stack top.
  fn peek(&self, offset: usize) -> &Value {
    unsafe { self.stack.get_unchecked(self.sp - offset - 1) }
  }

  // Raise a Runtime Error with massage.
  fn raise(self, msg: String) {
    eprintln!("RuntimeError: [line {}] {}", self.chunk.get_line_nu(self.ip), msg);
  }

  pub fn run(&mut self) {
    use OpCode::*;
    let mut ins;
    loop {
      ins = self.chunk.fetch(self.ip);

      println!("{}", ins);
      println!("== stack ==");
      for i in 0..self.sp {
        print!("[ {} ]", unsafe { self.stack.get_unchecked(i) })
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
        Neg => unsafe {
          match self.peek(0) {
            Value::Number(b) => *b = -*b,
            _ => self.raise("oprand must be number".into()),
          }
        },
        Add => binary!(self, +),
        Sub => binary!(self, -),
        Mul => binary!(self, *),
        Div => binary!(self, /),
      }
      self.ip += 1;
    }
  }
}
