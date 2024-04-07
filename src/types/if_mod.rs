use lib_ruby_parser::nodes::IfMod as InnerIfMod;

#[napi(object)]
pub struct IfMod {

}

impl From<InnerIfMod> for IfMod {
  fn from(node: InnerIfMod) -> Self {
    Self {

    }
  }
}
