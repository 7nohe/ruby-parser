use lib_ruby_parser::nodes::NthRef as InnerNthRef;

use crate::loc::Loc;

#[napi(object)]
pub struct NthRef {
  #[napi(ts_type = "'NthRef'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerNthRef> for NthRef {
  fn from(node: InnerNthRef) -> Self {
    Self {
      type_name: String::from("NthRef"),
      expression_l: node.expression_l.into(),
    }
  }
}
