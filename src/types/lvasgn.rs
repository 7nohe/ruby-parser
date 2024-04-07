use lib_ruby_parser::nodes::Lvasgn as InnerLvasgn;

#[napi(object)]
pub struct Lvasgn {

}

impl From<InnerLvasgn> for Lvasgn {
  fn from(node: InnerLvasgn) -> Self {
    Self {

    }
  }
}
