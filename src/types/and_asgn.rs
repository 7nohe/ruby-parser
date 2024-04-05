use lib_ruby_parser::nodes::AndAsgn as InnerAndAsgn;

use crate::{
  loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}
};



#[napi(object)]
pub struct AndAsgn {
  #[napi(ts_type = "'AndAsgn'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub recv: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerAndAsgn> for AndAsgn {
  fn from(node: InnerAndAsgn) -> Self {
    Self {
      type_name: String::from("AndAsgn"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: Loc::from(node.operator_l),
      expression_l: Loc::from(node.expression_l),
    }
  }
}
