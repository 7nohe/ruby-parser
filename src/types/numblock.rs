use lib_ruby_parser::nodes::Numblock as InnerNumblock;

#[napi(object)]
pub struct Numblock {
  #[napi(ts_type = "'Numblock'")]
  pub type_name: String,
}

impl From<InnerNumblock> for Numblock {
  fn from(node: InnerNumblock) -> Self {
    Self {
      type_name: String::from("Numblock"),
    }
  }
}
