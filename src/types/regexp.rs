use lib_ruby_parser::nodes::Regexp as InnerRegexp;

#[napi(object)]
pub struct Regexp {
  #[napi(ts_type = "'Regexp'")]
  pub type_name: String,
}

impl From<InnerRegexp> for Regexp {
  fn from(node: InnerRegexp) -> Self {
    Self {
      type_name: String::from("Regexp"),
    }
  }
}
