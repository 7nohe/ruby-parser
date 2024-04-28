use lib_ruby_parser::nodes::RegOpt as InnerRegOpt;

#[napi(object)]
pub struct RegOpt {
  #[napi(ts_type = "'RegOpt'")]
  pub type_name: String,
}

impl From<InnerRegOpt> for RegOpt {
  fn from(node: InnerRegOpt) -> Self {
    Self {
      type_name: String::from("RegOpt"),
    }
  }

}