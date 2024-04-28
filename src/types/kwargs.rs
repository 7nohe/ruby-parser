use lib_ruby_parser::nodes::Kwargs as InnerKwargs;

use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Kwargs {
  #[napi(ts_type = "'Kwargs'")]
  pub type_name: String,
  pub pairs: Vec<RubyNode>,
  pub expression_l: Loc,
}

impl From<InnerKwargs> for Kwargs {
  fn from(node: InnerKwargs) -> Self {
    Self {
      type_name: String::from("Kwargs"),
      pairs: node.pairs.into_iter().map(from_node).collect(),
      expression_l: node.expression_l.into(),
    }
  }
}
