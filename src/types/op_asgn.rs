use lib_ruby_parser::nodes::OpAsgn as InnerOpAsgn;

#[napi(object)]
pub struct OpAsgn {

}

impl From<InnerOpAsgn> for OpAsgn {
  fn from(node: InnerOpAsgn) -> Self {
    Self {

    }
  }
}
