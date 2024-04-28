use lib_ruby_parser::nodes::Ivar as InnerIvar;

use crate::loc::Loc;

#[napi(object)]
pub struct Ivar {
  #[napi(ts_type = "'Ivar'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerIvar> for Ivar {
  fn from(node: InnerIvar) -> Self {
    Self {
      type_name: String::from("Ivar"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}
