use lib_ruby_parser::nodes::Gvar as InnerGvar;

#[napi(object)]
pub struct Gvar {

}

impl From<InnerGvar> for Gvar {
  fn from(node: InnerGvar) -> Self {
    Self {

    }
  }
}
