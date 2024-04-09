use lib_ruby_parser::nodes::Heredoc as InnerHeredoc;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Heredoc {
  #[napi(ts_type = "'Heredoc'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub parts: Vec<RubyNode>,
  pub heredoc_body_l: Loc,
  pub heredoc_end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerHeredoc> for Heredoc {
  fn from(node: InnerHeredoc) -> Self {
    Self {
      type_name: String::from("Heredoc"),
      parts: node.parts.into_iter().map(|n| from_node(n)).collect(),
      heredoc_body_l: node.heredoc_body_l.into(),
      heredoc_end_l: node.heredoc_end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
