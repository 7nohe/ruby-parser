use lib_ruby_parser::nodes::XHeredoc as InnerXHeredoc;

#[napi(object)]
pub struct XHeredoc {
  #[napi(ts_type = "'XHeredoc'")]
  pub type_name: String,
}

impl From<InnerXHeredoc> for XHeredoc {
  fn from(node: InnerXHeredoc) -> Self {
    Self {
      type_name: String::from("XHeredoc"),
    }
  }
}
