use lib_ruby_parser::nodes::Rational as InnerRational;

#[napi(object)]
pub struct Rational {

}

impl From<InnerRational> for Rational {
  fn from(node: InnerRational) -> Self {
    Self {

    }
  }
}
