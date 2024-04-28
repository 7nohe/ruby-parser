use lib_ruby_parser::nodes::Pin as InnerPin;

#[napi(object)]
pub struct Pin {
  #[napi(ts_type = "'Pin'")]
  pub type_name: String,
}

impl From<InnerPin> for Pin {
  fn from(node: InnerPin) -> Self {
    Self {
      type_name: String::from("Pin"),
    }
  }
}
