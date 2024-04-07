use lib_ruby_parser::nodes::BackRef as InnerBackRef;

use crate::loc::Loc;

#[napi(object)]
pub struct BackRef {
  #[napi(ts_type = "'BackRef'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerBackRef> for BackRef {
  fn from(node: InnerBackRef) -> Self {
    Self {
      type_name: String::from("BackRef"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}
