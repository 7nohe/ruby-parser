use lib_ruby_parser::nodes::ArrayPatternWithTail as InnerArrayPatternWithTail;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct ArrayPatternWithTail {
  #[napi(ts_type = "'ArrayPatternWithTail'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub elements: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerArrayPatternWithTail> for ArrayPatternWithTail {
  fn from(node: InnerArrayPatternWithTail) -> Self {
    Self {
      type_name: String::from("ArrayPatternWithTail"),
      elements: node.elements.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
