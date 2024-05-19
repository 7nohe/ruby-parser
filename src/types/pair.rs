use lib_ruby_parser::nodes::Pair as InnerPair;

use crate::{
  loc::Loc,
  napi_box::NapiBox,
  napi_parser_result::{from_node, RubyNode},
};

#[napi(object)]
pub struct Pair {
  #[napi(ts_type = "'Pair'")]
  pub type_name: String,
  pub key: NapiBox<RubyNode>,
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerPair> for Pair {
  fn from(node: InnerPair) -> Self {
    Self {
      type_name: String::from("Pair"),
      key: NapiBox(Box::new(from_node(*node.key))),
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
