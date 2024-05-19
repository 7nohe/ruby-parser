use lib_ruby_parser::nodes::OrAsgn as InnerOrAsgn;

use crate::{
  loc::Loc,
  napi_box::NapiBox,
  napi_parser_result::{from_node, RubyNode},
};

#[napi(object)]
pub struct OrAsgn {
  #[napi(ts_type = "'OrAsgn'")]
  pub type_name: String,
  pub recv: NapiBox<RubyNode>,
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerOrAsgn> for OrAsgn {
  fn from(node: InnerOrAsgn) -> Self {
    Self {
      type_name: String::from("OrAsgn"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
