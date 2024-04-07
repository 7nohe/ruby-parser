use lib_ruby_parser::nodes::Yield as InnerYield;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Yield {
  #[napi(ts_type = "'Yield'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub args: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerYield> for Yield {
  fn from(node: InnerYield) -> Self {
    Self {
      type_name: String::from("Yield"),
      args: node.args.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
