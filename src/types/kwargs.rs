use lib_ruby_parser::nodes::Kwargs as InnerKwargs;

#[napi(object)]
pub struct Kwargs {

}

impl From<InnerKwargs> for Kwargs {
  fn from(node: InnerKwargs) -> Self {
    Self {

    }
  }
}
