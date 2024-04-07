use lib_ruby_parser::nodes::MatchPatternP as InnerMatchPatternP;

#[napi(object)]
pub struct MatchPatternP {

}

impl From<InnerMatchPatternP> for MatchPatternP {
  fn from(node: InnerMatchPatternP) -> Self {
    Self {

    }
  }
}
