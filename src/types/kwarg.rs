use lib_ruby_parser::nodes::Kwarg as InnerKwarg;
use crate::loc::Loc;

#[napi(object)]
pub struct Kwarg {
  #[napi(ts_type = "'Kwarg'")]
  pub type_name: String,
  pub name: String,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerKwarg> for Kwarg {
  fn from(node: InnerKwarg) -> Self {
    Self {
      type_name: String::from("Kwarg"),
      name: node.name,
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}