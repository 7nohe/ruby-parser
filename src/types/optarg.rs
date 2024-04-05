use lib_ruby_parser::nodes::Optarg as InnerOptarg;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Optarg {
  #[napi(ts_type = "'Optarg'")]
  pub type_name: String,
  pub name: String,
  #[napi(ts_type = "RubyNode")]
  pub default: NapiBox<RubyNode>,
  pub name_l: Loc,
  pub operator_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerOptarg> for Optarg {
  fn from(node: InnerOptarg) -> Self {
    Self {
      type_name: String::from("Optarg"),
      name: node.name,
      default: NapiBox(Box::new(from_node(*node.default))),
      name_l: node.name_l.into(),
      operator_l: node.operator_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}