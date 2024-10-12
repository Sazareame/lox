macro_rules! def_and_display_enum {
  ($name:ident, $($variant:ident),*) => {
    #[derive(Clone)]
    pub enum $name {
      $($variant),*
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::chunk::$name::*;
        match self {
          $($variant => write!(f, "{}", stringify!($variant).to_ascii_uppercase())),*
        }
      }
    }
  }
}

def_and_display_enum!(OpCode, Return);

pub struct Chunk {
  chunk: Vec<OpCode>,
}

impl Chunk {
  pub fn new() -> Self {
    Self { chunk: Vec::new() }
  }

  pub fn write_chunk(&mut self, code: OpCode) {
    self.chunk.push(code);
  }

  pub fn disassembley(&self, title: &str) {
    println!("== {} ==", title);
    self
      .chunk
      .iter()
      .for_each(|code| println!("{:04X} {}", (code.clone() as usize), code));
  }
}
