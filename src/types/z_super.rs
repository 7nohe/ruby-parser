use lib_ruby_parser::nodes::ZSuper as InnerZSuper;

#[napi(object)]
pub struct ZSuper {
  #[napi(ts_type = "'ZSuper'")]
  pub type_name: String,
}

impl From<InnerZSuper> for ZSuper {
  fn from(node: InnerZSuper) -> Self {
    Self {
      type_name: String::from("ZSuper"),
    }
  }
}
