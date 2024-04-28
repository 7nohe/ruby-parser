use lib_ruby_parser::nodes::Splat as InnerSplat;

#[napi(object)]
pub struct Splat {
  #[napi(ts_type = "'Splat'")]
  pub type_name: String,
}

impl From<InnerSplat> for Splat {
  fn from(node: InnerSplat) -> Self {
    Self {
      type_name: String::from("Splat"),
    }
  }
}
