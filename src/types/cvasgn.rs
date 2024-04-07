use lib_ruby_parser::nodes::Cvasgn as InnerCvasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Cvasgn {
  #[napi(ts_type = "'Cvasgn'")]
  pub type_name: String,
  pub name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub value: Option<NapiBox<RubyNode>>,
  pub name_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerCvasgn> for Cvasgn {
  fn from(node: InnerCvasgn) -> Self {
    Self {
      type_name: String::from("Cvasgn"),
      name: node.name,
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
