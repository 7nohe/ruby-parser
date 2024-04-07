use lib_ruby_parser::nodes::MatchCurrentLine as InnerMatchCurrentLine;

#[napi(object)]
pub struct MatchCurrentLine {

}

impl From<InnerMatchCurrentLine> for MatchCurrentLine {
  fn from(node: InnerMatchCurrentLine) -> Self {
    Self {

    }
  }
}
