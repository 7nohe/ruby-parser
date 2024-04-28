use lib_ruby_parser::nodes::Next as InnerNext;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Next {
  #[napi(ts_type = "'Next'")]
  pub type_name: String,
  pub args: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerNext> for Next {
  fn from(node: InnerNext) -> Self {
    Self {
      type_name: String::from("Next"),
      args: node.args.into_iter().map(from_node).collect(),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
