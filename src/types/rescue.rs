use lib_ruby_parser::nodes::Rescue as InnerRescue;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Rescue {
  #[napi(ts_type = "'Rescue'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub body: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode[]")]
  pub rescue_bodies: Vec<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub else_: Option<NapiBox<RubyNode>>,
  pub else_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerRescue> for Rescue {
  fn from(node: InnerRescue) -> Self {
    Self {
      type_name: String::from("Rescue"),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      rescue_bodies: node.rescue_bodies.into_iter().map(|n| from_node(n)).collect(),
      else_: node.else_.map(|n| NapiBox(Box::new(from_node(*n)))),
      else_l: node.else_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
