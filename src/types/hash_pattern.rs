use lib_ruby_parser::nodes::HashPattern as InnerHashPattern;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct HashPattern {
  #[napi(ts_type = "'HashPattern'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub elements: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerHashPattern> for HashPattern {
  fn from(node: InnerHashPattern) -> Self {
    Self {
      type_name: String::from("HashPattern"),
      elements: node.elements.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(|l| l.into()),
      end_l: node.end_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
