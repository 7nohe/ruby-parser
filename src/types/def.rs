use lib_ruby_parser::nodes::Def as InnerDef;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Def {
  #[napi(ts_type = "'Def'")]
  pub type_name: String,
  pub name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub args: Option<NapiBox<RubyNode>>,
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub name_l: Loc,
  pub end_l: Option<Loc>,
  pub assignment_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerDef> for Def {
  fn from(node: InnerDef) -> Self {
    Self {
      type_name: String::from("Def"),
      name: node.name,
      args: node.args.map(|v| NapiBox(Box::new(from_node(*v)))),
      body: node.body.map(|v| NapiBox(Box::new(from_node(*v)))),
      keyword_l: node.keyword_l.into(),
      name_l: node.name_l.into(),
      end_l: node.end_l.map(Into::into),
      assignment_l: node.assignment_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}