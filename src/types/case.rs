use lib_ruby_parser::nodes::Case as InnerCase;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Case {
  #[napi(ts_type = "'Case'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub expr: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode[]")]
  pub when_bodies: Vec<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub else_body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub else_l: Option<Loc>,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerCase> for Case {
  fn from(node: InnerCase) -> Self {
    Self {
      type_name: String::from("Case"),
      expr: node.expr.map(|n| NapiBox(Box::new(from_node(*n)))),
      when_bodies: node.when_bodies.into_iter().map(|n| from_node(n)).collect(),
      else_body: node.else_body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      else_l: node.else_l.map(|l| l.into()),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
