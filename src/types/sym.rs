use lib_ruby_parser::nodes::Sym as InnerSym;

use crate::{bytes::Bytes, loc::Loc};

#[napi(object)]
pub struct Sym {
  #[napi(ts_type = "'Sym'")]
  pub type_name: String,
  pub name: Bytes,
  pub begin_l: Option<Loc>,
  pub end_l: Option<Loc>,
  pub expression_l: Loc,
}

impl From<InnerSym> for Sym {
  fn from(node: InnerSym) -> Self {
    Self {
      type_name: String::from("Sym"),
      name: Bytes::from(node.name),
      begin_l: node.begin_l.map(Into::into),
      end_l: node.end_l.map(Into::into),
      expression_l: node.expression_l.into(),
    }
  }
}