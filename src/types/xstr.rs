use lib_ruby_parser::nodes::Xstr as InnerXstr;

#[napi(object)]
pub struct Xstr {
  #[napi(ts_type = "'Xstr'")]
  pub type_name: String,
}

impl From<InnerXstr> for Xstr {
  fn from(node: InnerXstr) -> Self {
    Self {
      type_name: String::from("Xstr"),
    }
  }
}
