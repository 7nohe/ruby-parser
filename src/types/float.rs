use lib_ruby_parser::nodes::Float as InnerFloat;

use crate::loc::Loc;

#[napi(object)]
pub struct Float {
  #[napi(ts_type = "'Float'")]
  pub type_name: String,
  pub value: String,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerFloat> for Float {
  fn from(node: InnerFloat) -> Self {
    Self {
      type_name: String::from("Float"),
      value: node.value,
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
