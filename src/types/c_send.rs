use lib_ruby_parser::nodes::CSend as InnerCSend;

use crate::{loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}};

#[napi(object)]
pub struct CSend {
  #[napi(ts_type = "'CSend'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode")]
  pub recv: NapiBox<RubyNode>,
  pub method_name: String,
  #[napi(ts_type = "RubyNode[]")]
  pub args: Vec<RubyNode>,
  pub dot_l: Loc,
  pub selector_l: Option<Loc>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerCSend> for CSend {
  fn from(node: InnerCSend) -> Self {
    Self {
      type_name: String::from("CSend"),
      recv: NapiBox(Box::new(from_node(*node.recv))),
      method_name: node.method_name,
      args: node.args.into_iter().map(from_node).collect(),
      dot_l: node.dot_l.into(),
      selector_l: node.selector_l.map(|l| l.into()),
      begin_l: node.begin_l.map(|l| l.into()),
      end_l: node.end_l.map(|l| l.into()),
      operator_l: node.operator_l.map(|l| l.into()),
      expression_l: node.expression_l.into(),
    }
  }
}
