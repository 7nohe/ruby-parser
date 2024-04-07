use lib_ruby_parser::nodes::When as InnerWhen;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct When {
  #[napi(ts_type = "'When'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub patterns: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub begin_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerWhen> for When {
  fn from(node: InnerWhen) -> Self {
    Self {
      type_name: String::from("When"),
      patterns: node.patterns.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
