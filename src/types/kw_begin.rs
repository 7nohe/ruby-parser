use lib_ruby_parser::nodes::KwBegin as InnerKwBegin;

#[napi(object)]
pub struct KwBegin {

}

impl From<InnerKwBegin> for KwBegin {
  fn from(node: InnerKwBegin) -> Self {
    Self {

    }
  }
}
