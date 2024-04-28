use lib_ruby_parser::nodes::MatchPatternP as InnerMatchPatternP;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchPatternP {
  #[napi(ts_type = "'MatchPatternP'")]
  pub type_name: String,
  pub value: NapiBox<RubyNode>,
  pub pattern: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchPatternP> for MatchPatternP {
  fn from(node: InnerMatchPatternP) -> Self {
    Self {
      type_name: String::from("MatchPatternP"),
      value: NapiBox(Box::new(from_node(*node.value))),
      pattern: NapiBox(Box::new(from_node(*node.pattern))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
