use lib_ruby_parser::nodes::False as InnerFalse;
use crate::loc::Loc;

#[napi(object)]
pub struct False {
  #[napi(ts_type = "'False'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerFalse> for False {
  fn from(node: InnerFalse) -> Self {
    Self {
      type_name: String::from("False"),
      expression_l: node.expression_l.into(),
    }
  }
}