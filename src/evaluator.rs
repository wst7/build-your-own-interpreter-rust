use crate::parser::expr::Expr;


pub struct Evaluator<'a> {
  ast: &'a Expr
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Expr) -> Self {
        Self { ast }
    }
    pub fn evaluate(&self) -> String {
      match self.ast {
          Expr::Literal(lit) => format!("{}", lit),
          _ => "".to_string()
      }
    }
}