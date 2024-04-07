use lib_ruby_parser::nodes::MatchVar as InnerMatchVar;

#[napi(object)]
pub struct MatchVar {

}

impl From<InnerMatchVar> for MatchVar {
  fn from(node: InnerMatchVar) -> Self {
    Self {

    }
  }
}
