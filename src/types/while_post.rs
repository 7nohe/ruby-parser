use lib_ruby_parser::nodes::WhilePost as InnerWhilePost;

#[napi(object)]
pub struct WhilePost {
  #[napi(ts_type = "'WhilePost'")]
  pub type_name: String,
}

impl From<InnerWhilePost> for WhilePost {
  fn from(node: InnerWhilePost) -> Self {
    Self {
      type_name: String::from("WhilePost"),
    }
  }
}
