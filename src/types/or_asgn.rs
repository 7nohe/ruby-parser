use lib_ruby_parser::nodes::OrAsgn as InnerOrAsgn;

#[napi(object)]
pub struct OrAsgn {
  #[napi(ts_type = "'OrAsgn'")]
  pub type_name: String,
}

impl From<InnerOrAsgn> for OrAsgn {
  fn from(node: InnerOrAsgn) -> Self {
    Self {
      type_name: String::from("OrAsgn"),
    }
  }
}
