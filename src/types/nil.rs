use lib_ruby_parser::nodes::Nil as InnerNil;
use crate::loc::Loc;

#[napi(object)]
pub struct Nil {
  #[napi(ts_type = "'Nil'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerNil> for Nil {
  fn from(node: InnerNil) -> Self {
    Self {
      type_name: String::from("Nil"),
      expression_l: node.expression_l.into(),
    }
  }
}