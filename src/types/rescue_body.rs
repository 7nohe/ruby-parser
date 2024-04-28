use lib_ruby_parser::nodes::RescueBody as InnerRescueBody;

#[napi(object)]
pub struct RescueBody {
  #[napi(ts_type = "'RescueBody'")]
  pub type_name: String,
}

impl From<InnerRescueBody> for RescueBody {
  fn from(node: InnerRescueBody) -> Self {
    Self {
      type_name: String::from("RescueBody"),
    }
  }
}
