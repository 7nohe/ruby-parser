use lib_ruby_parser::nodes::Alias as InnerAlias;

use crate::{
  loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}
};

#[napi(object)]
pub struct Alias {
  #[napi(ts_type = "'Alias'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub to: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode")]
  pub from: NapiBox<RubyNode>,
  pub keyword_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerAlias> for Alias {
  fn from(node: InnerAlias) -> Self {
    Self {
      type_name: String::from("Alias"),
      to: NapiBox(Box::new(from_node(*node.to))),
      from: NapiBox(Box::new(from_node(*node.from))),
      keyword_l: node.keyword_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
