use lib_ruby_parser::nodes::Pin as InnerPin;

#[napi(object)]
pub struct Pin {

}

impl From<InnerPin> for Pin {
  fn from(node: InnerPin) -> Self {
    Self {

    }
  }
}
