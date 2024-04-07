use lib_ruby_parser::nodes::Ensure as InnerEnsure;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Ensure {
  #[napi(ts_type = "'Ensure'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub ensure: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerEnsure> for Ensure {
  fn from(node: InnerEnsure) -> Self {
    Self {
      type_name: String::from("Ensure"),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      ensure: node.ensure.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
