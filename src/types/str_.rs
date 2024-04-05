use lib_ruby_parser::nodes::Str as InnerStr;

use crate::{bytes::Bytes, loc::Loc};

#[napi(object)]
pub struct Str {
  #[napi(ts_type = "'Str'")]
  pub type_name: String,
  pub value: Bytes,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerStr> for Str {
  fn from(node: InnerStr) -> Self {
    Self {
      type_name: String::from("Str"),
      value: Bytes::from(node.value),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
