use lib_ruby_parser::nodes::Pair as InnerPair;

#[napi(object)]
pub struct Pair {
  #[napi(ts_type = "'Pair'")]
  pub type_name: String,
}

impl From<InnerPair> for Pair {
  fn from(node: InnerPair) -> Self {
    Self {
      type_name: String::from("Pair"),
    }
  }
}
