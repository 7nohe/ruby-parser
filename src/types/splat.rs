use lib_ruby_parser::nodes::Splat as InnerSplat;

#[napi(object)]
pub struct Splat {

}

impl From<InnerSplat> for Splat {
  fn from(node: InnerSplat) -> Self {
    Self {

    }
  }
}
