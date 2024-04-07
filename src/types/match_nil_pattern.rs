use lib_ruby_parser::nodes::MatchNilPattern as InnerMatchNilPattern;

#[napi(object)]
pub struct MatchNilPattern {

}

impl From<InnerMatchNilPattern> for MatchNilPattern {
  fn from(node: InnerMatchNilPattern) -> Self {
    Self {

    }
  }
}
