use lib_ruby_parser::nodes::Irange as InnerIrange;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Irange {
  #[napi(ts_type = "'Irange'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub left: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub right: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerIrange> for Irange {
  fn from(node: InnerIrange) -> Self {
    Self {
      type_name: String::from("Irange"),
      left: node.left.map(|n| NapiBox(Box::new(from_node(*n)))),
      right: node.right.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
