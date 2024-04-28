use lib_ruby_parser::nodes::Retry as InnerRetry;

#[napi(object)]
pub struct Retry {
  #[napi(ts_type = "'Retry'")]
  pub type_name: String,
}

impl From<InnerRetry> for Retry {
  fn from(node: InnerRetry) -> Self {
    Self {
      type_name: String::from("Retry"),
    }
  }
}
