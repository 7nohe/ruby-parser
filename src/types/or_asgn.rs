use lib_ruby_parser::nodes::OrAsgn as InnerOrAsgn;

#[napi(object)]
pub struct OrAsgn {

}

impl From<InnerOrAsgn> for OrAsgn {
  fn from(node: InnerOrAsgn) -> Self {
    Self {

    }
  }
}
