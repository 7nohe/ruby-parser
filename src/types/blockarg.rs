use lib_ruby_parser::nodes::Blockarg as InnerBlockarg;
use crate::loc::Loc;

#[napi(object)]
pub struct Blockarg {
  #[napi(ts_type = "'Blockarg'")]
  pub type_name: String,
  pub name: Option<String>,
  pub operator_l: Loc,
  pub name_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerBlockarg> for Blockarg {
  fn from(node: InnerBlockarg) -> Self {
    Self {
      type_name: String::from("Blockarg"),
      name: node.name,
      operator_l: node.operator_l.into(),
      name_l: node.name_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}