use lib_ruby_parser::nodes::Index as InnerIndex;

#[napi(object)]
pub struct Index {

}

impl From<InnerIndex> for Index {
  fn from(node: InnerIndex) -> Self {
    Self {

    }
  }
}
