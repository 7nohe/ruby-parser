use lib_ruby_parser::nodes::IFlipFlop as InnerIFlipFlop;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct IFlipFlop {
  #[napi(ts_type = "'IFlipFlop'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub left: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub right: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc
}

impl From<InnerIFlipFlop> for IFlipFlop {
  fn from(node: InnerIFlipFlop) -> Self {
    Self {
      type_name: String::from("IFlipFlop"),
      left: node.left.map(|n| NapiBox(Box::new(from_node(*n)))),
      right: node.right.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
