use lib_ruby_parser::nodes::BlockPass as InnerBlockPass;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct BlockPass {
  #[napi(ts_type = "'BlockPass'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub value: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerBlockPass> for BlockPass {
  fn from(node: InnerBlockPass) -> Self {
    Self {
      type_name: String::from("BlockPass"),
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
