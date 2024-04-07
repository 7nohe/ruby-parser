use lib_ruby_parser::nodes::XHeredoc as InnerXHeredoc;

#[napi(object)]
pub struct XHeredoc {

}

impl From<InnerXHeredoc> for XHeredoc {
  fn from(node: InnerXHeredoc) -> Self {
    Self {

    }
  }
}
