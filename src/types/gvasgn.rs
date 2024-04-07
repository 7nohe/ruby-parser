use lib_ruby_parser::nodes::Gvasgn as InnerGvasgn;

#[napi(object)]
pub struct Gvasgn {

}

impl From<InnerGvasgn> for Gvasgn {
  fn from(node: InnerGvasgn) -> Self {
    Self {

    }
  }
}
