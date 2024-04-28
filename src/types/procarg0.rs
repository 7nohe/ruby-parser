use lib_ruby_parser::nodes::Procarg0 as InnerProcarg0;

#[napi(object)]
pub struct Procarg0 {
  #[napi(ts_type = "'Procarg0'")]
  pub type_name: String,
}

impl From<InnerProcarg0> for Procarg0 {
  fn from(node: InnerProcarg0) -> Self {
    Self {
      type_name: String::from("Procarg0"),
    }
  }
}
