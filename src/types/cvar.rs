use lib_ruby_parser::nodes::Cvar as InnerCvar;

use crate::loc::Loc;

#[napi(object)]
pub struct Cvar {
  #[napi(ts_type = "'Cvar'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerCvar> for Cvar {
  fn from(node: InnerCvar) -> Self {
    Self {
      type_name: String::from("Cvar"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}
