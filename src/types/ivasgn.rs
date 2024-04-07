use lib_ruby_parser::nodes::Ivasgn as InnerIvasgn;

#[napi(object)]
pub struct Ivasgn {

}

impl From<InnerIvasgn> for Ivasgn {
  fn from(node: InnerIvasgn) -> Self {
    Self {

    }
  }
}
