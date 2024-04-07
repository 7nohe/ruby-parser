use lib_ruby_parser::nodes::Masgn as InnerMasgn;

#[napi(object)]
pub struct Masgn {

}

impl From<InnerMasgn> for Masgn {
  fn from(node: InnerMasgn) -> Self {
    Self {

    }
  }
}
