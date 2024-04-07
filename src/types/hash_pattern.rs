use lib_ruby_parser::nodes::HashPattern as InnerHashPattern;

#[napi(object)]
pub struct HashPattern {

}

impl From<InnerHashPattern> for HashPattern {
  fn from(node: InnerHashPattern) -> Self {
    Self {

    }
  }
}
