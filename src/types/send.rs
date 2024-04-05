use crate::{
  loc::Loc, napi_box::NapiBox, napi_parser_result::{from_node, RubyNode}
};
use lib_ruby_parser::nodes::Send as InnerSend;

#[napi(object)]
pub struct Send {
  #[napi(ts_type = "'Send'")]
  pub type_name: String,
  #[napi(ts_type = "RubyNode | undefined")]
  pub recv: Option<NapiBox<RubyNode>>,
  pub method_name: String,
  pub args: Vec<RubyNode>,
  pub dot_l: Option<Loc>,
  pub selector_l: Option<Loc>,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub operator_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerSend> for Send {
  fn from(node: InnerSend) -> Self {
    Self {
      type_name: String::from("Send"),
      recv: node.recv.map(|v| NapiBox(Box::new(from_node(*v)))),
      method_name: node.method_name.to_string(),
      args: node.args.into_iter().map(from_node).collect(),
      dot_l: node.dot_l.map(Into::into),
      selector_l: node.selector_l.map(Into::into),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      operator_l: node.operator_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
