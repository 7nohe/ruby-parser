use lib_ruby_parser::nodes::ForwardArg as InnerForwardArg;
use crate::loc::Loc;

#[napi(object)]
pub struct ForwardArg {
  #[napi(ts_type = "'ForwardArg'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerForwardArg> for ForwardArg {
  fn from(node: InnerForwardArg) -> Self {
    Self {
      type_name: String::from("ForwardArg"),
      expression_l: node.expression_l.into(),
    }
  }
}