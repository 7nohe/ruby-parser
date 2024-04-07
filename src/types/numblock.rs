use lib_ruby_parser::nodes::Numblock as InnerNumblock;

#[napi(object)]
pub struct Numblock {

}

impl From<InnerNumblock> for Numblock {
  fn from(node: InnerNumblock) -> Self {
    Self {

    }
  }
}
