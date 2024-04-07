use lib_ruby_parser::nodes::Or as InnerOr;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Or {
  #[napi(ts_type = "'Or'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub lhs: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub rhs: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerOr> for Or {
  fn from(node: InnerOr) -> Self {
    Self {
      type_name: String::from("Or"),
      lhs: NapiBox(Box::new(from_node(*node.lhs))),
      rhs: NapiBox(Box::new(from_node(*node.rhs))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
