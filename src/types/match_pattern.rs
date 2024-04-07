use lib_ruby_parser::nodes::MatchPattern as InnerMatchPattern;

#[napi(object)]
pub struct MatchPattern {

}

impl From<InnerMatchPattern> for MatchPattern {
  fn from(node: InnerMatchPattern) -> Self {
    Self {

    }
  }
}
