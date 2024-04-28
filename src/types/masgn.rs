use lib_ruby_parser::nodes::Masgn as InnerMasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Masgn {
  #[napi(ts_type = "'Masgn'")]
  pub type_name: String,
  pub lhs: NapiBox<RubyNode>,
  pub rhs: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMasgn> for Masgn {
  fn from(node: InnerMasgn) -> Self {
    Self {
      type_name: String::from("Masgn"),
      lhs: NapiBox(Box::new(from_node(*node.lhs))),
      rhs: NapiBox(Box::new(from_node(*node.rhs))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
