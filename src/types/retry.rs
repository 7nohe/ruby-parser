use lib_ruby_parser::nodes::Retry as InnerRetry;

#[napi(object)]
pub struct Retry {

}

impl From<InnerRetry> for Retry {
  fn from(node: InnerRetry) -> Self {
    Self {

    }
  }
}
