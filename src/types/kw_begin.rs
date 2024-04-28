use lib_ruby_parser::nodes::KwBegin as InnerKwBegin;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct KwBegin {
  #[napi(ts_type = "'KwBegin'")]
  pub type_name: String,
  pub statements: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerKwBegin> for KwBegin {
  fn from(node: InnerKwBegin) -> Self {
    Self {
      type_name: String::from("KwBegin"),
      statements: node.statements.into_iter().map(from_node).collect(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
