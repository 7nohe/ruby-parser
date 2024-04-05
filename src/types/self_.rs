use lib_ruby_parser::nodes::Self_ as InnerSelf;
use crate::loc::Loc;

#[napi(object)]
pub struct RubySelf {
  #[napi(ts_type = "'RubySelf'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerSelf> for RubySelf {
  fn from(node: InnerSelf) -> Self {
    Self {
      type_name: String::from("RubySelf"),
      expression_l: node.expression_l.into(),
    }
  }
}