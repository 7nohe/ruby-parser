use lib_ruby_parser::nodes::NthRef as InnerNthRef;

#[napi(object)]
pub struct NthRef {

}

impl From<InnerNthRef> for NthRef {
  fn from(node: InnerNthRef) -> Self {
    Self {

    }
  }
}
