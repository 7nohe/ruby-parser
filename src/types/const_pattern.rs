use lib_ruby_parser::nodes::ConstPattern as InnerConstPattern;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct ConstPattern {
  #[napi(ts_type = "'ConstPattern'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub const_: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub pattern: NapiBox<RubyNode>,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerConstPattern> for ConstPattern {
  fn from(node: InnerConstPattern) -> Self {
    Self {
      type_name: String::from("ConstPattern"),
      const_: NapiBox(Box::new(from_node(*node.const_))),
      pattern: NapiBox(Box::new(from_node(*node.pattern))),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
