use lib_ruby_parser::nodes::Block as InnerBlock;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Block {
  #[napi(ts_type = "'Block'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub call: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub args: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerBlock> for Block {
  fn from(node: InnerBlock) -> Self {
    Self {
      type_name: String::from("Block"),
      call: NapiBox(Box::new(from_node(*node.call))),
      args: node.args.map(|n| NapiBox(Box::new(from_node(*n)))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
