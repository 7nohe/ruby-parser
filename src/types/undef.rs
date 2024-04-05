use lib_ruby_parser::nodes::Undef as InnerUndef;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};


#[napi(object)]
pub struct Undef {
  #[napi(ts_type = "'Undef'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub names: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerUndef> for Undef {
  fn from(node: InnerUndef) -> Self {
    Self {
      type_name: String::from("Undef"),
      names: node.names.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}