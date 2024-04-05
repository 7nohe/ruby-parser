use lib_ruby_parser::nodes::Preexe as InnerPreexe;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Preexe {
  #[napi(ts_type = "'Preexe'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerPreexe> for Preexe {
  fn from(node: InnerPreexe) -> Self {
    Self {
      type_name: String::from("Preexe"),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}