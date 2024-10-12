#[macro_use]
mod macro_def{
macro_rules! clean {
    ($typ:ty) => { _ };
}
macro_rules! def_opcode {
  ($name:ident, $($variant:ident$(($($carry:ty),+))?),+) => {
    #[repr(C)]
    pub enum $name {
      $($variant$(($($carry),+))?),*
    }

    impl $name {
      pub fn to_int(&self) -> u32 {
        unsafe{*<*const _>::from(self).cast()}
      }
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::chunk::$name::*;
        match self {
          $($variant$(($(clean!($carry)),+))? => write!(f, "{:04}  {:-10}", self.to_int(), stringify!($variant).to_ascii_uppercase())),+
        }
      }
    }
  }
}
}


def_opcode!(OpCode, Return, Constant(u8));
use crate::value::Value;

// #[repr(C)]
// pub enum OpCode {
//   Return,
//   Constant(u8),
// }
// 
// impl OpCode {
//   pub fn to_int(&self) -> u32 {
//     unsafe{*<*const _>::from(self).cast()}
//   }
// }
// 
// impl std::fmt::Display for OpCode {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     use OpCode::*;
//     match self{
//       Return => write!(f, "{:04} OP_RETURN", self.to_int()),
//       Constant(i) => wri
//     }
//   }
// }

/// Constant Pool used to store constant define by OP_CONSTANT,  
/// The OP_CONSTANT could access the value it refers to by the u8 it carried as index.
struct ConstantPool{
  constants: Vec<Value>,
}

impl ConstantPool{
  pub fn new() -> Self{
    Self{constants: Vec::new()}
  }

  /// Add a constant to the pool, then return its index in the underlaying data buffer.  
  /// This mothod is now use u8 as return value, which means that the index's max value is 255.
  pub fn add_constant(&mut self, val: Value) -> u8{
    self.constants.push(val);
    (self.constants.len() - 1).try_into().unwrap()
  }

  /// Retrievl the constant value via index.  
  /// This method assert that the given index is always valid and do no bound-checking.  
  /// Also, the index is represented by u8.
  pub fn get_constant(&self, index: u8) -> Value{
    unsafe{*self.constants.get_unchecked(index as usize)}
  }
}

/// Chunk used to store OpCodes and a constant pool.
pub struct Chunk {
  chunks: Vec<OpCode>,
  constants: ConstantPool,
  /// A parallel array store the line number the associated opcode comes from.
  lines: Vec<u8>,
}

impl Chunk {
  pub fn new() -> Self {
    Self { 
      chunks: Vec::new(),
      constants: ConstantPool::new(),
      lines: Vec::new(),
    }
  }

  /// Add an OpCode into the underlying data buffer hold by Chunk.
  pub fn write_chunk(&mut self, code: OpCode, line: u8) {
    self.chunks.push(code);
    self.lines.push(line);
  }

  /// Add a constant value into the constant pool it contains, then return the index of that constant in the pool.  
  /// This return value is what is carried by the OpCode Enum.
  pub fn write_constant(&mut self, val: Value) -> u8 {
    self.constants.add_constant(val)
  }

  /// Display the opcodes in Chunk by lines, with additional information if exists.
  pub fn disassembly(&self, title: &str) {
    println!("== {} ==", title);
    println!("{:16}{}\n", "INSTRUCTION", "LINE");
    self
      .chunks
      .iter().zip(&self.lines)
      .for_each(|(ins, line)| self.disassembly_ins(ins, *line));
  }

  fn disassembly_ins(&self, ins: &OpCode, line: u8) {
    use OpCode::*;
    match ins{
      // (code) (line number) (constant index) (constant value)
      Constant(i) => println!("{}  {}  {}'{}", ins, line, i, self.constants.get_constant(*i)),
      _ => println!("{}  {}", ins, line),
    }
  }
}
