use lib_ruby_parser::nodes::IfTernary as InnerIfTernary;

#[napi(object)]
pub struct IfTernary {

}

impl From<InnerIfTernary> for IfTernary {
  fn from(node: InnerIfTernary) -> Self {
    Self {

    }
  }
}
