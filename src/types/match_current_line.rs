use lib_ruby_parser::nodes::MatchCurrentLine as InnerMatchCurrentLine;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchCurrentLine {
  #[napi(ts_type = "'MatchCurrentLine'")]
  pub type_name: String,
  pub re: NapiBox<RubyNode>,
  pub expression_l: Loc,
}

impl From<InnerMatchCurrentLine> for MatchCurrentLine {
  fn from(node: InnerMatchCurrentLine) -> Self {
    Self {
      type_name: String::from("MatchCurrentLine"),
      re: NapiBox(Box::new(from_node(*node.re))),
      expression_l: node.expression_l.into(),
    }
  }
}
