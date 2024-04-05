use lib_ruby_parser::nodes::Cbase as InnerCbase;
use crate::loc::Loc;

#[napi(object)]
pub struct Cbase {
  #[napi(ts_type = "'Cbase'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerCbase> for Cbase {
  fn from(node: InnerCbase) -> Self {
    Self {
      type_name: String::from("Cbase"),
      expression_l: node.expression_l.into(),
    }
  }
}