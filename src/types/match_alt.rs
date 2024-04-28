use lib_ruby_parser::nodes::MatchAlt as InnerMatchAlt;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchAlt {
  #[napi(ts_type = "'MatchAlt'")]
  pub type_name: String,
  pub lhs: NapiBox<RubyNode>,
  pub rhs: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchAlt> for MatchAlt {
  fn from(node: InnerMatchAlt) -> Self {
    Self {
      type_name: String::from("MatchAlt"),
      lhs: NapiBox(Box::new(from_node(*node.lhs))),
      rhs: NapiBox(Box::new(from_node(*node.rhs))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
