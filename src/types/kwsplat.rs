use lib_ruby_parser::nodes::Kwsplat as InnerKwsplat;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Kwsplat {
  #[napi(ts_type = "'Kwsplat'")]
  pub type_name: String,
  pub value: NapiBox<RubyNode>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerKwsplat> for Kwsplat {
  fn from(node: InnerKwsplat) -> Self {
    Self {
      type_name: String::from("Kwsplat"),
      value: NapiBox(Box::new(from_node(*node.value))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
