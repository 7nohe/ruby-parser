use lib_ruby_parser::nodes::IfTernary as InnerIfTernary;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct IfTernary {
  #[napi(ts_type = "'IfTernary'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub cond: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub if_true: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub if_false: NapiBox<RubyNode>,
  pub question_l: Loc,
  pub colon_l: Loc,
  pub expression_l: Loc
}

impl From<InnerIfTernary> for IfTernary {
  fn from(node: InnerIfTernary) -> Self {
    Self {
      type_name: String::from("IfTernary"),
      cond: NapiBox(Box::new(from_node(*node.cond))),
      if_true: NapiBox(Box::new(from_node(*node.if_true))),
      if_false: NapiBox(Box::new(from_node(*node.if_false))),
      question_l: node.question_l.into(),
      colon_l: node.colon_l.into(),
      expression_l: node.expression_l.into()
    }
  }
}
