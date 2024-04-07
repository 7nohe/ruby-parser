use lib_ruby_parser::nodes::Erange as InnerErange;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Erange {
  #[napi(ts_type = "'Erange'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub left: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub right: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc
}

impl From<InnerErange> for Erange {
  fn from(node: InnerErange) -> Self {
    Self {
      type_name: String::from("Erange"),
      left: node.left.map(|n| NapiBox(Box::new(from_node(*n)))),
      right: node.right.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
