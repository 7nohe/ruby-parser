use lib_ruby_parser::nodes::Hash as InnerHash;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Hash {
  #[napi(ts_type = "'Hash'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub pairs: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerHash> for Hash {
  fn from(node: InnerHash) -> Self {
    Self {
      type_name: String::from("Hash"),
      pairs: node.pairs.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
