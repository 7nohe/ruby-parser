use lib_ruby_parser::nodes::Defined as InnerDefined;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Defined {
  #[napi(ts_type = "'Defined'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub value: NapiBox<RubyNode>,
  pub keyword_l: Loc,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerDefined> for Defined {
  fn from(node: InnerDefined) -> Self {
    Self {
      type_name: String::from("Defined"),
      value: NapiBox(Box::new(from_node(*node.value))),
      keyword_l: node.keyword_l.into(),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
