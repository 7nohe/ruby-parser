use lib_ruby_parser::nodes::MatchAlt as InnerMatchAlt;

#[napi(object)]
pub struct MatchAlt {

}

impl From<InnerMatchAlt> for MatchAlt {
  fn from(node: InnerMatchAlt) -> Self {
    Self {

    }
  }
}
