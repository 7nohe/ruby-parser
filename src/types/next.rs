use lib_ruby_parser::nodes::Next as InnerNext;

#[napi(object)]
pub struct Next {

}

impl From<InnerNext> for Next {
  fn from(node: InnerNext) -> Self {
    Self {

    }
  }
}
