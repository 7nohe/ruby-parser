use lib_ruby_parser::nodes::Kwrestarg as InnerKwrestarg;
use crate::loc::Loc;

#[napi(object)]
pub struct Kwrestarg {
  #[napi(ts_type = "'Kwrestarg'")]
  pub type_name: String,
  pub name: Option<String>,
  pub operator_l: Loc,
  pub name_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerKwrestarg> for Kwrestarg {
  fn from(node: InnerKwrestarg) -> Self {
    Self {
      type_name: String::from("Kwrestarg"),
      name: node.name,
      operator_l: node.operator_l.into(),
      name_l: node.name_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}