use lib_ruby_parser::nodes::OpAsgn as InnerOpAsgn;

use crate::{
  loc::Loc,
  napi_box::NapiBox,
  napi_parser_result::{from_node, RubyNode},
};

#[napi(object)]
pub struct OpAsgn {
  #[napi(ts_type = "'OpAsgn'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub recv: NapiBox<RubyNode>,
  pub operator: String,
  #[napi(ts_type = "RubyNode")]
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerOpAsgn> for OpAsgn {
  fn from(node: InnerOpAsgn) -> Self {
    Self {
      type_name: String::from("OpAsgn"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      operator: node.operator,
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
