use lib_ruby_parser::nodes::Ivasgn as InnerIvasgn;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Ivasgn {
  #[napi(ts_type = "'Ivasgn'")]
  pub type_name: String,
  pub name: String,
  pub value: Option<NapiBox<RubyNode>>,
  pub name_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerIvasgn> for Ivasgn {
  fn from(node: InnerIvasgn) -> Self {
    Self {
      type_name: String::from("Ivasgn"),
      name: node.name,
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
