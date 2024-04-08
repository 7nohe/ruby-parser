use lib_ruby_parser::nodes::Gvar as InnerGvar;

use crate::loc::Loc;

#[napi(object)]
pub struct Gvar {
  #[napi(ts_type = "'Gvar'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerGvar> for Gvar {
  fn from(node: InnerGvar) -> Self {
    Self {
      type_name: String::from("Gvar"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}
