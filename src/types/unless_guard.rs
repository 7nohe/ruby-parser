use lib_ruby_parser::nodes::UnlessGuard as InnerUnlessGuard;

#[napi(object)]
pub struct UnlessGuard {
  #[napi(ts_type = "'UnlessGuard'")]
  pub type_name: String,
}

impl From<InnerUnlessGuard> for UnlessGuard {
  fn from(node: InnerUnlessGuard) -> Self {
    Self {
      type_name: String::from("UnlessGuard"),
    }
  }
}
