use lib_ruby_parser::nodes::Lvasgn as InnerLvasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Lvasgn {
  #[napi(ts_type = "'Lvasgn'")]
  pub type_name: String,
  pub value: Option<NapiBox<RubyNode>>,
  pub name_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerLvasgn> for Lvasgn {
  fn from(node: InnerLvasgn) -> Self {
    Self {
      type_name: String::from("Lvasgn"),
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
