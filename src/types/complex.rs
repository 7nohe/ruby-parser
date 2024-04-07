use lib_ruby_parser::nodes::Complex as InnerComplex;

use crate::loc::Loc;

#[napi(object)]
pub struct Complex {
  #[napi(ts_type = "'Complex'")]
  pub type_name: String,
  pub value: String,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerComplex> for Complex {
  fn from(node: InnerComplex) -> Self {
    Self {
      type_name: String::from("Complex"),
      value: node.value,
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
