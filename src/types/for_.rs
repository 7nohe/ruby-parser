use lib_ruby_parser::nodes::For as InnerFor;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct For {
  #[napi(ts_type = "'For'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub iterator: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub iteratee: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub operator_l: Loc,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerFor> for For {
  fn from(node: InnerFor) -> Self {
    Self {
      type_name: String::from("For"),
      iterator: NapiBox(Box::new(from_node(*node.iterator))),
      iteratee: NapiBox(Box::new(from_node(*node.iteratee))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      operator_l: node.operator_l.into(),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
