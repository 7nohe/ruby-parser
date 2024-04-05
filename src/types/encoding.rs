use lib_ruby_parser::nodes::Encoding as InnerEncoding;
use crate::loc::Loc;

#[napi(object)]
pub struct Encoding {
  #[napi(ts_type = "'Encoding'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerEncoding> for Encoding {
  fn from(node: InnerEncoding) -> Self {
    Self {
      type_name: String::from("Encoding"),
      expression_l: node.expression_l.into(),
    }
  }
}