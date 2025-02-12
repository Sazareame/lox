#[macro_export]
macro_rules! clean {
  ($typ:ty) => {
    _
  };
}

#[macro_export]
macro_rules! def_opcode {
($name:ident, $($variant:ident$(($($carry:ty),+))?),+) => {
  #[derive(Clone, Copy)]
  #[repr(C)]
  pub enum $name {
    $($variant$(($($carry),+))?),*
  }

  impl $name {
    pub fn for_int_print(&self) -> u32 {
      unsafe{*<*const _>::from(self).cast()}
    }
  }

  impl std::fmt::Display for $name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      use $name::*;
      match self {
        $($variant$(($($crate::clean!($carry)),+))? => write!(f, "{:04}  {:-10}", self.for_int_print(), stringify!($variant).to_ascii_uppercase())),+
      }
    }
  }
}
}

#[macro_export]
macro_rules! def_tokentype {
  ($($typ:ident),*) => {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum TokenType {
      $($typ),*
    }

    impl ::std::fmt::Display for TokenType {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        match self {
          $($typ => write!(f, "{}", stringify!($typ).to_ascii_uppercase())),*
        }
      }
    }
  }
}
