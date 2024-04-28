use lib_ruby_parser::nodes::MatchNilPattern as InnerMatchNilPattern;

use crate::loc::Loc;

#[napi(object)]
pub struct MatchNilPattern {
  #[napi(ts_type = "'MatchNilPattern'")]
  pub type_name: String,
  pub operator_l: Loc,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchNilPattern> for MatchNilPattern {
  fn from(node: InnerMatchNilPattern) -> Self {
    Self {
      type_name: String::from("MatchNilPattern"),
      operator_l: node.operator_l.into(),
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
