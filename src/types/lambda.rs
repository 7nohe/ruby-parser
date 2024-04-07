use lib_ruby_parser::nodes::Lambda as InnerLambda;

use crate::loc::Loc;

#[napi(object)]
pub struct Lambda {
  #[napi(ts_type = "'Lambda'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerLambda> for Lambda {
  fn from(node: InnerLambda) -> Self {
    Self {
      type_name: String::from("Lambda"),
      expression_l: node.expression_l.into(),
    }
  }
}
