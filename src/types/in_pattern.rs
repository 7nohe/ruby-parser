use lib_ruby_parser::nodes::InPattern as InnerInPattern;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct InPattern {
  #[napi(ts_type = "'InPattern'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub pattern: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub guard: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub begin_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerInPattern> for InPattern {
  fn from(node: InnerInPattern) -> Self {
    Self {
      type_name: String::from("InPattern"),
      pattern: NapiBox(Box::new(from_node(*node.pattern))),
      guard: node.guard.map(|n| NapiBox(Box::new(from_node(*n)))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
