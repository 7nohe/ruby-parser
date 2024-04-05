use lib_ruby_parser::nodes::Postexe as InnerPostexe;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Postexe {
  #[napi(ts_type = "'Postexe'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerPostexe> for Postexe {
  fn from(node: InnerPostexe) -> Self {
    Self {
      type_name: String::from("Postexe"),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}