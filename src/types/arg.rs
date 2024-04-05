use lib_ruby_parser::nodes::Arg as InnerArg;
use crate::loc::Loc;

#[napi(object)]
pub struct Arg {
  #[napi(ts_type = "'Arg'")]
  pub type_name: String,
  pub name: String,
  pub expression_l: Loc,
}

impl From<InnerArg> for Arg {
  fn from(node: InnerArg) -> Self {
    Self {
      type_name: String::from("Arg"),
      name: node.name,
      expression_l: node.expression_l.into(),
    }
  }
}