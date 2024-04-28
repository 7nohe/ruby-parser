use lib_ruby_parser::nodes::MatchPattern as InnerMatchPattern;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchPattern {
  #[napi(ts_type = "'MatchPattern'")]
  pub type_name: String,
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchPattern> for MatchPattern {
  fn from(node: InnerMatchPattern) -> Self {
    Self {
      type_name: String::from("MatchPattern"),
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
