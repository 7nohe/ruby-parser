use lib_ruby_parser::nodes::Restarg as InnerRestarg;
use crate::loc::Loc;


#[napi(object)]
pub struct Restarg {
  #[napi(ts_type = "'Restarg'")]
  pub type_name: String,
  pub name: Option<String>,
  pub operator_l: Loc,
  pub name_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerRestarg> for Restarg {
  fn from(node: InnerRestarg) -> Self {
    Self {
      type_name: String::from("Restarg"),
      name: node.name,
      operator_l: node.operator_l.into(),
      name_l: node.name_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}