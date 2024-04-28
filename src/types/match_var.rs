use lib_ruby_parser::nodes::MatchVar as InnerMatchVar;

use crate::loc::Loc;

#[napi(object)]
pub struct MatchVar {
  #[napi(ts_type = "'MatchVar'")]
  pub type_name: String,
  pub name_l: Loc,
  pub expression_l: Loc,
}

impl From<InnerMatchVar> for MatchVar {
  fn from(node: InnerMatchVar) -> Self {
    Self {
      type_name: String::from("MatchVar"),
      name_l: node.name_l.into(),
      expression_l: node.expression_l.into(),
    }
  }
}
