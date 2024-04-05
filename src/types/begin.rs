use lib_ruby_parser::nodes::Begin as InnerBegin;
use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Begin {
  #[napi(ts_type = "'Begin'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub statements: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerBegin> for Begin {
  fn from(node: InnerBegin) -> Self {
    Self {
      type_name: String::from("Begin"),
      statements: node.statements.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(|l| l.into()),
      end_l: node.end_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}