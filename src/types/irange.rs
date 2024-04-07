use lib_ruby_parser::nodes::Irange as InnerIrange;

#[napi(object)]
pub struct Irange {

}

impl From<InnerIrange> for Irange {
  fn from(node: InnerIrange) -> Self {
    Self {

    }
  }
}
