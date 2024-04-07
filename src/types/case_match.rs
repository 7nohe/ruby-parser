use lib_ruby_parser::nodes::CaseMatch as InnerCaseMatch;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct CaseMatch {
  #[napi(ts_type = "'CaseMatch'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub expr: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode[]")]
  pub in_bodies: Vec<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub else_body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub else_l: Option<Loc>,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerCaseMatch> for CaseMatch {
  fn from(node: InnerCaseMatch) -> Self {
    Self {
      type_name: String::from("CaseMatch"),
      expr: NapiBox(Box::new(from_node(*node.expr))),
      in_bodies: node.in_bodies.into_iter().map(|n| from_node(n)).collect(),
      else_body: node.else_body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      else_l: node.else_l.map(|l| l.into()),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
