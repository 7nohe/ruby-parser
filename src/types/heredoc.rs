use lib_ruby_parser::nodes::Heredoc as InnerHeredoc;

#[napi(object)]
pub struct Heredoc {

}

impl From<InnerHeredoc> for Heredoc {
  fn from(node: InnerHeredoc) -> Self {
    Self {

    }
  }
}
