use lib_ruby_parser::nodes::OpAsgn as InnerOpAsgn;

#[napi(object)]
pub struct OpAsgn {
  #[napi(ts_type = "'OpAsgn'")]
  pub type_name: String,
}

impl From<InnerOpAsgn> for OpAsgn {
  fn from(node: InnerOpAsgn) -> Self {
    Self {
      type_name: String::from("OpAsgn"),
    }
  }
}
