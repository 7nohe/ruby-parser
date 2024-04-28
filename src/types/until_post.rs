use lib_ruby_parser::nodes::UntilPost as InnerUntilPost;

#[napi(object)]
pub struct UntilPost {
  #[napi(ts_type = "'UntilPost'")]
  pub type_name: String,
}

impl From<InnerUntilPost> for UntilPost {
  fn from(node: InnerUntilPost) -> Self {
    Self {
      type_name: String::from("UntilPost"),
    }
  }
}
