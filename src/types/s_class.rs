use lib_ruby_parser::nodes::SClass as InnerSClass;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct SClass {
  #[napi(ts_type = "'SClass'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub expr: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub operator_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerSClass> for SClass {
  fn from(node: InnerSClass) -> Self {
    Self {
      type_name: String::from("SClass"),
      expr: NapiBox(Box::new(from_node(*node.expr))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      operator_l: node.operator_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}