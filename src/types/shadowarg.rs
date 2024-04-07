use lib_ruby_parser::nodes::Shadowarg as InnerShadowarg;

#[napi(object)]
pub struct Shadowarg {

}

impl From<InnerShadowarg> for Shadowarg {
  fn from(node: InnerShadowarg) -> Self {
    Self {

    }
  }
}
