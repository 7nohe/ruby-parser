use lib_ruby_parser::nodes::MatchWithLvasgn as InnerMatchWithLvasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchWithLvasgn {
  #[napi(ts_type = "'MatchWithLvasgn'")]
  pub type_name: String,
  pub re: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchWithLvasgn> for MatchWithLvasgn {
  fn from(node: InnerMatchWithLvasgn) -> Self {
    Self {
      type_name: String::from("MatchWithLvasgn"),
      re: NapiBox(Box::new(from_node(*node.re))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
