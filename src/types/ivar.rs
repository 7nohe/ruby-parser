use lib_ruby_parser::nodes::Ivar as InnerIvar;

#[napi(object)]
pub struct Ivar {

}

impl From<InnerIvar> for Ivar {
  fn from(node: InnerIvar) -> Self {
    Self {

    }
  }
}
