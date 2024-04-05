use lib_ruby_parser::nodes::Defs as InnerDefs;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Defs {
  #[napi(ts_type = "'Defs'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub definee: NapiBox<RubyNode>,
  pub name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub args: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub operator_l: Loc,
  pub name_l: Loc,
  pub assignment_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerDefs> for Defs {
  fn from(node: InnerDefs) -> Self {
    Self {
      type_name: String::from("Defs"),
      definee: NapiBox(Box::new(from_node(*node.definee))),
      name: node.name,
      args: node.args.map(|v| NapiBox(Box::new(from_node(*v)))),
      body: node.body.map(|n| NapiBox(Box::new(from_node(*n)))),
      keyword_l: node.keyword_l.into(),
      operator_l: node.operator_l.into(),
      name_l: node.name_l.into(),
      assignment_l: node.assignment_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}