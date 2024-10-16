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

macro_rules! binary{
  ($vm:ident, $op: tt) => {
    {
    if !$vm.peek(0).is_number() || !$vm.peek(1).is_number() {
      $vm.raise("operand must be numbers".into());
      return;
    }
    let rhs = $vm.pop().as_number().unwrap();
    let lhs = $vm.pop().as_number().unwrap();
    $vm.push($crate::value::Value::Number(lhs $op rhs));
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

  // Raise a Runtime Error with massage, reset the stack.
  fn raise(&mut self, msg: String) {
    eprintln!("RuntimeError: [line {}] {}", self.chunk.get_line_nu(self.ip), msg);
    self.sp = 0;
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
        Neg => match self.peek(0) {
          Value::Number(_) => {
            let n = self.pop().as_number().unwrap();
            self.push(Value::Number(-n));
          }
          _ => {
            self.raise("oprand must be number".into());
            return;
          }
        },
        True => self.push(Value::Boolean(true)),
        False => self.push(Value::Boolean(false)),
        Nil => self.push(Value::Nil),
        Add => binary!(self, +),
        Sub => binary!(self, -),
        Mul => binary!(self, *),
        Div => binary!(self, /),
      }
      self.ip += 1;
    }
  }
}
