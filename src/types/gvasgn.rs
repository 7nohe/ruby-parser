use lib_ruby_parser::nodes::Gvasgn as InnerGvasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Gvasgn {
  #[napi(ts_type = "'Gvasgn'")]
  pub type_name: String,
  pub name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub value: Option<NapiBox<RubyNode>>,
  pub name_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc
}

impl From<InnerGvasgn> for Gvasgn {
  fn from(node: InnerGvasgn) -> Self {
    Self {
      type_name: String::from("Gvasgn"),
      name: node.name,
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.map(|l| l.into()),
      expression_l: node.expression_l.into()
    }
  }
}
