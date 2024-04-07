use lib_ruby_parser::nodes::ZSuper as InnerZSuper;

#[napi(object)]
pub struct ZSuper {

}

impl From<InnerZSuper> for ZSuper {
  fn from(node: InnerZSuper) -> Self {
    Self {

    }
  }
}
