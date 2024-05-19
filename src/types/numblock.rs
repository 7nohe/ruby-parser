use lib_ruby_parser::nodes::Numblock as InnerNumblock;

use crate::{
  loc::Loc,
  napi_box::NapiBox,
  napi_parser_result::{from_node, RubyNode},
};

#[napi(object)]
pub struct Numblock {
  #[napi(ts_type = "'Numblock'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub call: NapiBox<RubyNode>,
  pub numargs: u8,
  #[napi(ts_type = "RubyNode")]
  pub body: NapiBox<RubyNode>,
  pub begin_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerNumblock> for Numblock {
  fn from(node: InnerNumblock) -> Self {
    Self {
      type_name: String::from("Numblock"),
      call: NapiBox(Box::new(from_node(*node.call))),
      numargs: node.numargs,
      body: NapiBox(Box::new(from_node(*node.body))),
      begin_l: node.begin_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
