use lib_ruby_parser::nodes::Regexp as InnerRegexp;

#[napi(object)]
pub struct Regexp {

}

impl From<InnerRegexp> for Regexp {
  fn from(node: InnerRegexp) -> Self {
    Self {

    }
  }
}
