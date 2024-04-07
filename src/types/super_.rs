use lib_ruby_parser::nodes::Super as InnerSuper;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Super {
  #[napi(ts_type = "'Super'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub args: Vec<RubyNode>,
  pub keyword_l: Loc,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerSuper> for Super {
  fn from(node: InnerSuper) -> Self {
    Self {
      type_name: String::from("Super"),
      args: node.args.into_iter().map(|n| from_node(n)).collect(),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
