use lib_ruby_parser::nodes::Module as InnerModule;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Module {
  #[napi(ts_type = "'Module'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub name: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerModule> for Module {
  fn from(node: InnerModule) -> Self {
    Self {
      type_name: String::from("Module"),
      name: NapiBox(Box::new(from_node(*node.name))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}