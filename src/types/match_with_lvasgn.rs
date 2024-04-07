use lib_ruby_parser::nodes::MatchWithLvasgn as InnerMatchWithLvasgn;

#[napi(object)]
pub struct MatchWithLvasgn {

}

impl From<InnerMatchWithLvasgn> for MatchWithLvasgn {
  fn from(node: InnerMatchWithLvasgn) -> Self {
    Self {

    }
  }
}
