use lib_ruby_parser::nodes::Lvar as InnerLvar;
use crate::loc::Loc;

#[napi(object)]
pub struct Lvar {
  #[napi(ts_type = "'Lvar'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerLvar> for Lvar {
  fn from(node: InnerLvar) -> Self {
    Self {
      type_name: String::from("Lvar"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}