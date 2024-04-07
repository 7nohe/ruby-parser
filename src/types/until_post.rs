use lib_ruby_parser::nodes::UntilPost as InnerUntilPost;

#[napi(object)]
pub struct UntilPost {

}

impl From<InnerUntilPost> for UntilPost {
  fn from(node: InnerUntilPost) -> Self {
    Self {

    }
  }
}
