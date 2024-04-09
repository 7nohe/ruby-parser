use lib_ruby_parser::nodes::Index as InnerIndex;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Index {
  #[napi(ts_type = "'Index'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub recv: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode[]")]
  pub indexes: Vec<RubyNode>,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerIndex> for Index {
  fn from(node: InnerIndex) -> Self {
    Self {
      type_name: String::from("Index"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      indexes: node.indexes.into_iter().map(|n| from_node(n)).collect(),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
