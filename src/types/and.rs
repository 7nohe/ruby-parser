use lib_ruby_parser::nodes::And as InnerAnd;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct And {
  #[napi(ts_type = "'And'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub lhs: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub rhs: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerAnd> for And {
  fn from(node: InnerAnd) -> Self {
    Self {
      type_name: String::from("And"),
      lhs: NapiBox(Box::new(from_node(*node.lhs))),
      rhs: NapiBox(Box::new(from_node(*node.rhs))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
