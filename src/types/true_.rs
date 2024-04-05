use lib_ruby_parser::nodes::True as InnerTrue;
use crate::loc::Loc;

#[napi(object)]
pub struct True {
  #[napi(ts_type = "'True'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerTrue> for True {
  fn from(node: InnerTrue) -> Self {
    Self {
      type_name: String::from("True"),
      expression_l: node.expression_l.into(),
    }
  }
}