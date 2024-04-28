use lib_ruby_parser::nodes::MatchAs as InnerMatchAs;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchAs {
  #[napi(ts_type = "'MatchAs'")]
  pub type_name: String,
  pub value: NapiBox<RubyNode>,
  pub as_: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchAs> for MatchAs {
  fn from(node: InnerMatchAs) -> Self {
    Self {
      type_name: String::from("MatchAs"),
      value: NapiBox(Box::new(from_node(*node.value))),
      as_: NapiBox(Box::new(from_node(*node.as_))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
