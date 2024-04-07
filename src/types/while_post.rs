use lib_ruby_parser::nodes::WhilePost as InnerWhilePost;

#[napi(object)]
pub struct WhilePost {

}

impl From<InnerWhilePost> for WhilePost {
  fn from(node: InnerWhilePost) -> Self {
    Self {

    }
  }
}
