use lib_ruby_parser::nodes::Kwsplat as InnerKwsplat;

#[napi(object)]
pub struct Kwsplat {

}

impl From<InnerKwsplat> for Kwsplat {
  fn from(node: InnerKwsplat) -> Self {
    Self {

    }
  }
}
