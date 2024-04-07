use lib_ruby_parser::nodes::Until as InnerUntil;

#[napi(object)]
pub struct Until {

}

impl From<InnerUntil> for Until {
  fn from(node: InnerUntil) -> Self {
    Self {

    }
  }
}
