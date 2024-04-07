use lib_ruby_parser::nodes::UnlessGuard as InnerUnlessGuard;

#[napi(object)]
pub struct UnlessGuard {

}

impl From<InnerUnlessGuard> for UnlessGuard {
  fn from(node: InnerUnlessGuard) -> Self {
    Self {

    }
  }
}
