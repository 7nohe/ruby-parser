use lib_ruby_parser::nodes::Break as InnerBreak;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Break {
  #[napi(ts_type = "'Break'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub args: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerBreak> for Break {
  fn from(node: InnerBreak) -> Self {
    Self {
      type_name: String::from("Break"),
      args: node.args.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
