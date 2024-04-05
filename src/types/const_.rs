use lib_ruby_parser::nodes::Const as InnerConst;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Const {
  #[napi(ts_type = "'Const'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub scope: Option<NapiBox<RubyNode>>,
  pub name: String,
  pub double_colon_l: Option<Loc>,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerConst> for Const {
  fn from(node: InnerConst) -> Self {
    Self {
      type_name: String::from("Const"),
      scope: node.scope.map(|v| NapiBox(Box::new(from_node(*v)))),
      name: node.name,
      double_colon_l: node.double_colon_l.map(Into::into),
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}