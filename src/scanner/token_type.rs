use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Not, Neg};
use std::cmp::{PartialOrd, PartialEq};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType{
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  IDENTIFIER, STRING, NUMBER,

  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF,
}

#[derive(Clone)]
pub enum Object{
  Num(f64),
  Str(String),
  Nil,
  Bool(bool),
}

impl Display for Object{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self{
      Object::Num(n) => write!(f, "{}", n),
      Object::Bool(b) => write!(f, "{}", b),
      Object::Nil => write!(f, "Nil"),
      Object::Str(s) => write!(f, "{}", s),
    }   
  }
}

impl Object{
	pub fn to_number(&self) -> f64{
		match &self{
			Object::Num(n) => *n,
			Object::Bool(_) => panic!("converting boolean into number"),
			Object::Nil => 0f64, 
			Object::Str(s) => s.parse().expect("unable to convert string into number"),
		}
	}
}

impl Add for Object{
	type Output = Self;
	fn add(self, rhs: Self) -> Self{
		if let Self::Str(ls) = self{
			if let Self::Str(rs) = rhs{
				return Object::Str(ls + &rs);
			}else{return Object::Nil;} // meaningless deadcode qaq
		}else{
		let l = self.to_number();
		let r = rhs.to_number();
		Object::Num(l + r)
	}
	}
}

impl Sub for Object{
	type Output = Self;	
	fn sub(self, rhs: Self) -> Self{
		let l = self.to_number();
		let r = rhs.to_number();
		Object::Num(l - r)	
	}
}

impl Mul for Object{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self{
		let l = self.to_number();
		let r = rhs.to_number();
		Object::Num(l * r)
	}
}

impl Div for Object{
	type Output = Self;
	fn div(self, rhs: Self) -> Self{
		let l = self.to_number();
		let r = rhs.to_number();
		Object::Num(l / r)
	}	
}

impl Neg for Object{
	type Output = Self;
	fn neg(self) -> Self{
		if let Self::Num(n) = self{
			Self::Num(-n)
		}else{
			self
		}		
	}
}

impl Not for Object{
	type Output = Self;
	fn not(self) -> Self{
		match self{
			Self::Nil => Self::Bool(true),
			Self::Bool(b) => Self::Bool(!b),
			_ => Self::Bool(false),
		}
	}
}

impl PartialEq for Object{
	fn eq(&self, other: &Self) -> bool{
		match self{
			Self::Nil => other == &Self::Nil,
			Self::Bool(b) => other == &Self::Bool(*b),
			Self::Num(n) => other == &Self::Num(*n),
			Self::Str(s) => {
				if let Self::Str(rs) = other{
					return s == rs;
				}
				return false;
			}
		}
	}
}

impl PartialOrd for Object{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let lhs = self.to_number();
		let rhs = other.to_number();
		lhs.partial_cmp(&rhs)
	}
}