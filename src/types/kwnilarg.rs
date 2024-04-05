use lib_ruby_parser::nodes::Kwnilarg as InnerKwnilarg;
use crate::loc::Loc;

#[napi(object)]
pub struct Kwnilarg {
  #[napi(ts_type = "'Kwnilarg'")]
  pub type_name: String,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerKwnilarg> for Kwnilarg {
  fn from(node: InnerKwnilarg) -> Self {
    Self {
      type_name: String::from("Kwnilarg"),
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}