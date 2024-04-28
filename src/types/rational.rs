use lib_ruby_parser::nodes::Rational as InnerRational;

#[napi(object)]
pub struct Rational {
  #[napi(ts_type = "'Rational'")]
  pub type_name: String,
}

impl From<InnerRational> for Rational {
  fn from(node: InnerRational) -> Self {
    Self {
      type_name: String::from("Rational"),
    }
  }
}
