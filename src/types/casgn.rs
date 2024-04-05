use lib_ruby_parser::nodes::Casgn as InnerCasgn;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Casgn {
  #[napi(ts_type = "'Casgn'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub scope: Option<NapiBox<RubyNode>>,
  pub name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub value: Option<NapiBox<RubyNode>>,
  pub double_colon_l: Option<Loc>,
  pub name_l: Loc,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerCasgn> for Casgn {
  fn from(node: InnerCasgn) -> Self {
    Self {
      type_name: String::from("Casgn"),
      scope: node.scope.map(|n| NapiBox(Box::new(from_node(*n)))),
      name: node.name,
      value: node.value.map(|n| NapiBox(Box::new(from_node(*n)))),
      double_colon_l: node.double_colon_l.map(Into::into),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}