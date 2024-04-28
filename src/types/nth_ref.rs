use lib_ruby_parser::nodes::NthRef as InnerNthRef;

#[napi(object)]
pub struct NthRef {
  #[napi(ts_type = "'NthRef'")]
  pub type_name: String,
}

impl From<InnerNthRef> for NthRef {
  fn from(node: InnerNthRef) -> Self {
    Self {
      type_name: String::from("NthRef"),
    }
  }
}
