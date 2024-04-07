use lib_ruby_parser::nodes::While as InnerWhile;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct While {
  #[napi(ts_type = "'While'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub cond: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerWhile> for While {
  fn from(node: InnerWhile) -> Self {
    Self {
      type_name: String::from("While"),
      cond: NapiBox(Box::new(from_node(*node.cond))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.map(|l| l.into()),
      end_l: node.end_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
