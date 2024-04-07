use lib_ruby_parser::nodes::MatchRest as InnerMatchRest;

#[napi(object)]
pub struct MatchRest {

}

impl From<InnerMatchRest> for MatchRest {
  fn from(node: InnerMatchRest) -> Self {
    Self {

    }
  }
}
