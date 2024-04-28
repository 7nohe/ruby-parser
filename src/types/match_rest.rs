use lib_ruby_parser::nodes::MatchRest as InnerMatchRest;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct MatchRest {
  #[napi(ts_type = "'MatchRest'")]
  pub type_name: String,
  pub name: Option<NapiBox<RubyNode>>,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchRest> for MatchRest {
  fn from(node: InnerMatchRest) -> Self {
    Self {
      type_name: String::from("MatchRest"),
      name: node.name.map(|n| NapiBox(Box::new(from_node(*n)))),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
