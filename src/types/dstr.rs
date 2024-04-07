use lib_ruby_parser::nodes::Dstr as InnerDstr;

use crate::loc::Loc;

#[napi(object)]
pub struct Dstr {
  #[napi(ts_type = "'Dstr'")]
  pub type_name: String,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerDstr> for Dstr {
  fn from(node: InnerDstr) -> Self {
    Self {
      type_name: String::from("Dstr"),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}
