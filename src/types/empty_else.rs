use lib_ruby_parser::nodes::EmptyElse as InnerEmptyElse;

use crate::loc::Loc;

#[napi(object)]
pub struct EmptyElse {
  #[napi(ts_type = "'EmptyElse'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerEmptyElse> for EmptyElse {
  fn from(node: InnerEmptyElse) -> Self {
    Self {
      type_name: String::from("EmptyElse"),
      expression_l: node.expression_l.into(),
    }
  }
}
