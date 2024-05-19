use lib_ruby_parser::nodes::Pin as InnerPin;

use crate::{
  loc::Loc,
  napi_box::NapiBox,
  napi_parser_result::{from_node, RubyNode},
};

#[napi(object)]
pub struct Pin {
  #[napi(ts_type = "'Pin'")]
  pub type_name: String,
  pub var: NapiBox<RubyNode>,
  pub selector_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerPin> for Pin {
  fn from(node: InnerPin) -> Self {
    Self {
      type_name: String::from("Pin"),
      var: NapiBox(Box::new(from_node(*node.var))),
      selector_l: node.selector_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
