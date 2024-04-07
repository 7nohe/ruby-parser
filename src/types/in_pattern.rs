use lib_ruby_parser::nodes::InPattern as InnerInPattern;

#[napi(object)]
pub struct InPattern {

}

impl From<InnerInPattern> for InPattern {
  fn from(node: InnerInPattern) -> Self {
    Self {

    }
  }
}
