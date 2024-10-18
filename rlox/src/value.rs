use std::rc::Rc;
#[derive(Clone)]
pub enum Value {
  Number(f64),
  Boolean(bool),
  Str(Rc<String>),
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
      Value::Str(s) => write!(f, "{}", s),
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

  pub fn is_number(&self) -> bool {
    matches!(self, Value::Number(_))
  }

  pub fn as_bool(&self) -> Option<bool> {
    if let Self::Boolean(b) = self {
      Some(*b)
    } else {
      None
    }
  }

  pub fn is_string(&self) -> bool {
    matches!(self, Value::Str(_))
  }

  pub fn as_string(&self) -> Option<Rc<String>> {
    if let Value::Str(s) = self {
      Some(s.clone())
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
      Self::Str(s) => *s == other.as_string().unwrap(),
    }
  }
}
