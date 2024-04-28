use lib_ruby_parser::nodes::Until as InnerUntil;

#[napi(object)]
pub struct Until {
  #[napi(ts_type = "'Until'")]
  pub type_name: String,
}

impl From<InnerUntil> for Until {
  fn from(node: InnerUntil) -> Self {
    Self {
      type_name: String::from("Until"),
    }
  }
}
