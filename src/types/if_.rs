use lib_ruby_parser::nodes::If as InnerIf;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct If {
  #[napi(ts_type = "'If'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub cond: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub if_true: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub if_false: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub begin_l: Loc,
  pub else_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerIf> for If {
  fn from(node: InnerIf) -> Self {
    Self {
      type_name: String::from("If"),
      cond: NapiBox(Box::new(from_node(*node.cond))),
      if_true: node.if_true.map(|n| NapiBox(Box::new(from_node(*n)))),
      if_false: node.if_false.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.into(),
      else_l: node.else_l.map(|l| l.into()),
      end_l: node.end_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
