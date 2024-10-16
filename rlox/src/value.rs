#[derive(Clone, Copy)]
pub enum Value {
  Number(f64),
  Boolean(bool),
  Nil,
}

impl std::default::Default for Value {
  fn default() -> Self {
    Self::Nil
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Number(n) => write!(f, "{}", n),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Nil => write!(f, "nil"),
    }
  }
}

impl Value {
  pub fn as_number(&self) -> Option<f64> {
    if let Self::Number(n) = self {
      Some(*n)
    } else {
      None
    }
  }

  pub fn as_bool(&self) -> Option<bool> {
    if let Self::Boolean(b) = self {
      Some(*b)
    } else {
      None
    }
  }

  /// Whether the Value is false in Lox.
  pub fn is_false(&self) -> bool {
    match self {
      Self::Nil => true,
      Self::Boolean(b) => !b,
      _ => false,
    }
  }

  /// Whether two Lox Value are equal.
  pub fn equals(&self, other: &Self) -> bool {
    if std::mem::discriminant(self) != std::mem::discriminant(other) {
      return false;
    }
    match self {
      Self::Nil => true,
      Self::Number(n) => *n == other.as_number().unwrap(),
      Self::Boolean(b) => *b == other.as_bool().unwrap(),
    }
  }
}
