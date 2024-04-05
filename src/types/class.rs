use lib_ruby_parser::nodes::Class as InnerClass;
use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct Class {
  #[napi(ts_type = "'Class'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub name: NapiBox<RubyNode>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub superclass: Option<NapiBox<RubyNode>>,
  #[napi(ts_type = "RubyNode | undefined")]
  pub body: Option<NapiBox<RubyNode>>,
  pub keyword_l: Loc,
  pub operator_l: Option<Loc>,
  pub end_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerClass> for Class {
  fn from(node: InnerClass) -> Self {
    Self {
      type_name: String::from("Class"),
      name: NapiBox(Box::new(from_node(*node.name))),
      superclass: node.superclass.map(|n| NapiBox(Box::new(from_node(*n)))),
      body: node.body.map(|v| NapiBox(Box::new(from_node(*v)))),
      keyword_l: node.keyword_l.into(),
      operator_l: node.operator_l.map(Into::into),
      end_l: node.end_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}