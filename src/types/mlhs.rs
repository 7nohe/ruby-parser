use lib_ruby_parser::nodes::Mlhs as InnerMlhs;
use crate::{loc::Loc, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Mlhs {
  #[napi(ts_type = "'Mlhs'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub items: Vec<RubyNode>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerMlhs> for Mlhs {
  fn from(node: InnerMlhs) -> Self {
    Self {
      type_name: String::from("Mlhs"),
      items: node.items.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}