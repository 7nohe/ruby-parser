use lib_ruby_parser::nodes::Line as InnerLine;
use crate::loc::Loc;

#[napi(object)]
pub struct Line {
  #[napi(ts_type = "'Line'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerLine> for Line {
  fn from(node: InnerLine) -> Self {
    Self {
      type_name: String::from("Line"),
      expression_l: node.expression_l.into(),
    }
  }
}