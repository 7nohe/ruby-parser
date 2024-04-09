use lib_ruby_parser::nodes::IfGuard as InnerIfGuard;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct IfGuard {
  #[napi(ts_type = "'IfGuard'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub cond: NapiBox<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc
}

impl From<InnerIfGuard> for IfGuard {
  fn from(node: InnerIfGuard) -> Self {
    Self {
      type_name: String::from("IfGuard"),
      cond: NapiBox(Box::new(from_node(*node.cond))),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
