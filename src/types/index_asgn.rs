use lib_ruby_parser::nodes::IndexAsgn as InnerIndexAsgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct IndexAsgn {
  #[napi(ts_type = "'IndexAsgn'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub recv: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode[]")]
  pub indexes: Vec<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub value: Option<NapiBox<RubyNode>>,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerIndexAsgn> for IndexAsgn {
  fn from(node: InnerIndexAsgn) -> Self {
    Self {
      type_name: String::from("IndexAsgn"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      indexes: node.indexes.into_iter().map(from_node).collect(),
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      operator_l: node.operator_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
