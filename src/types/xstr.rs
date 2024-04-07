use lib_ruby_parser::nodes::Xstr as InnerXstr;

#[napi(object)]
pub struct Xstr {

}

impl From<InnerXstr> for Xstr {
  fn from(node: InnerXstr) -> Self {
    Self {

    }
  }
}
