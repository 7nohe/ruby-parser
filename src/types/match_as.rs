use lib_ruby_parser::nodes::MatchAs as InnerMatchAs;

#[napi(object)]
pub struct MatchAs {

}

impl From<InnerMatchAs> for MatchAs {
  fn from(node: InnerMatchAs) -> Self {
    Self {

    }
  }
}
