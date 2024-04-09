use lib_ruby_parser::nodes::IfMod as InnerIfMod;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct IfMod {
  #[napi(ts_type = "'IfMod'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub cond: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub if_true: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub if_false: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub expression_l: Loc
}

impl From<InnerIfMod> for IfMod {
  fn from(node: InnerIfMod) -> Self {
    Self {
      type_name: String::from("IfMod"),
      cond: NapiBox(Box::new(from_node(*node.cond))),
      if_true: node.if_true.map(|n| NapiBox(Box::new(from_node(*n)))),
      if_false: node.if_false.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
