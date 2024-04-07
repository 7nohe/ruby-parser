use lib_ruby_parser::nodes::EFlipFlop as InnerEFlipFlop;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct EFlipFlop {
  #[napi(ts_type = "'EFlipFlop'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub left: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub right: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc
}

impl From<InnerEFlipFlop> for EFlipFlop {
  fn from(node: InnerEFlipFlop) -> Self {
    Self {
      type_name: String::from("EFlipFlop"),
      left: node.left.map(|n| NapiBox(Box::new(from_node(*n)))),
      right: node.right.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
