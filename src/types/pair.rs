use lib_ruby_parser::nodes::Pair as InnerPair;

#[napi(object)]
pub struct Pair {

}

impl From<InnerPair> for Pair {
  fn from(node: InnerPair) -> Self {
    Self {

    }
  }
}
