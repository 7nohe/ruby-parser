use lib_ruby_parser::nodes::Args as InnerArgs;
use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Args {
  #[napi(ts_type = "'Args'")]
  pub type_name: String,
  pub args: Vec<RubyNode>,
  pub expression_l: Loc,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
}

impl From<InnerArgs> for Args {
  fn from(node: InnerArgs) -> Self {
    Self {
      type_name: String::from("Args"),
      args: node.args.into_iter().map(from_node).collect(),
      expression_l: node.expression_l.into(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
    }
  }
}