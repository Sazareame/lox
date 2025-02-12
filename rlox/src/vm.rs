use crate::chunk::*;
use crate::value::Value;
use std::rc::Rc;

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
  ($vm:ident, $op: tt, $typ:ident) => {
    {
    if !$vm.peek(0).is_number() || !$vm.peek(1).is_number() {
      $vm.raise("operand must be numbers".into());
      return;
    }
    let rhs = $vm.pop().as_number().unwrap();
    let lhs = $vm.pop().as_number().unwrap();
    $vm.push($crate::value::Value::$typ(lhs $op rhs));
    }
  }
}

impl VM {
  pub fn new(chunk: Chunk) -> Self {
    Self {
      chunk,
      stack: std::array::from_fn(|_| Value::Nil),
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
    println!("== RUNNING VM ==");
    loop {
      ins = self.chunk.fetch(self.ip);
      println!("EXECUING INSTRUCTION: {}", ins);
      match ins {
        Return => {
          println!("{}\n", self.pop());
          return;
        }
        Constant(val) => {
          let constant = self.chunk.get_constant(val.into());
          self.push(constant);
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
        Not => {
          let a = self.pop().is_false();
          self.push(Value::Boolean(a));
        }
        True => self.push(Value::Boolean(true)),
        False => self.push(Value::Boolean(false)),
        Nil => self.push(Value::Nil),
        Add => {
          if self.peek(0).is_number() && self.peek(1).is_number() {
            let rhs = self.pop().as_number().unwrap();
            let lhs = self.pop().as_number().unwrap();
            self.push(Value::Number(lhs + rhs));
          } else if self.peek(0).is_string() && self.peek(1).is_string() {
            let rhs = (*self.pop().as_string().unwrap()).clone();
            let lhs = (*self.pop().as_string().unwrap()).clone();
            self.push(Value::Str(Rc::new(lhs + &rhs)))
          }
        }
        Sub => binary!(self, -, Number),
        Mul => binary!(self, *, Number),
        Div => binary!(self, /, Number),
        Greater => binary!(self, >, Boolean),
        Less => binary!(self, <, Boolean),
        Equal => {
          let rhs = self.pop();
          let lhs = self.pop();
          self.push(Value::Boolean(lhs.equals(&rhs)));
        }
      }
      self.ip += 1;
      println!("== STACK ==");
      if self.sp == 0 {
        print!("EMPTY");
      }
      for i in 0..self.sp {
        print!("[ {} ]", unsafe { self.stack.get_unchecked(i) })
      }
      println!();
    }
  }
}
