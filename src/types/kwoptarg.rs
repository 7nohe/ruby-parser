use lib_ruby_parser::nodes::Kwoptarg as InnerKwoptarg;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Kwoptarg {
  #[napi(ts_type = "'Kwoptarg'")]
  pub type_name: String,
  pub name: String,
  #[napi(ts_type = "RubyNode")]
  pub default: NapiBox<RubyNode>,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerKwoptarg> for Kwoptarg {
  fn from(node: InnerKwoptarg) -> Self {
    Self {
      type_name: String::from("Kwoptarg"),
      name: node.name,
      default: NapiBox(Box::new(from_node(*node.default))),
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}