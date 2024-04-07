use lib_ruby_parser::nodes::Dsym as InnerDsym;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Dsym {
  #[napi(ts_type = "'Dsym'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub parts: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerDsym> for Dsym {
  fn from(node: InnerDsym) -> Self {
    Self {
      type_name: String::from("Dsym"),
      parts: node.parts.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
