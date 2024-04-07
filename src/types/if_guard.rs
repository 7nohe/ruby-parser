use lib_ruby_parser::nodes::IfGuard as InnerIfGuard;

#[napi(object)]
pub struct IfGuard {

}

impl From<InnerIfGuard> for IfGuard {
  fn from(node: InnerIfGuard) -> Self {
    Self {

    }
  }
}
