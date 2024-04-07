use lib_ruby_parser::nodes::Return as InnerReturn;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Return {
  #[napi(ts_type = "'Return'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub args: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerReturn> for Return {
  fn from(node: InnerReturn) -> Self {
    Self {
      type_name: String::from("Return"),
      args: node.args.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
