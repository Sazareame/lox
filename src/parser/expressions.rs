use crate::scanner::token::Token;
use crate::scanner::token_type::Object;

trait Expr{}

struct Binary{
  left: Box<dyn Expr>,
  operator: Token,
  right: Box<dyn Expr>,
}

struct Grouping{
  expression: Box<dyn Expr>,
}

struct Literal{
  value: Object,
}

struct Unary{
  operator: Token,
  right: Box<dyn Expr>,
}

impl Expr for Binary{}
impl Expr for Grouping{}
impl Expr for Literal{}
impl Expr for Unary{}