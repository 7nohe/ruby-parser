use lib_ruby_parser::nodes::Shadowarg as InnerShadowarg;

#[napi(object)]
pub struct Shadowarg {
  #[napi(ts_type = "'Shadowarg'")]
  pub type_name: String,
}

impl From<InnerShadowarg> for Shadowarg {
  fn from(node: InnerShadowarg) -> Self {
    Self {
      type_name: String::from("Shadowarg"),
    }
  }
}
