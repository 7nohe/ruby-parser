use lib_ruby_parser::nodes::RescueBody as InnerRescueBody;

#[napi(object)]
pub struct RescueBody {

}

impl From<InnerRescueBody> for RescueBody {
  fn from(node: InnerRescueBody) -> Self {
    Self {

    }
  }
}
