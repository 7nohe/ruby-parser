use lib_ruby_parser::nodes::Int as InnerInt;
use crate::loc::Loc;


#[napi(object)]
pub struct Int {
  #[napi(ts_type = "'Int'")]
  pub type_name: String,
  pub value: String,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerInt> for Int {
  fn from(node: InnerInt) -> Self {
    Self {
      type_name: String::from("Int"),
      value: node.value,
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}