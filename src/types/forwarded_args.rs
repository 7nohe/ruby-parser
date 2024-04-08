use lib_ruby_parser::nodes::ForwardedArgs as InnerForwardedArgs;

use crate::loc::Loc;

#[napi(object)]
pub struct ForwardedArgs {
  #[napi(ts_type = "'ForwardedArgs'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerForwardedArgs> for ForwardedArgs {
  fn from(node: InnerForwardedArgs) -> Self {
    Self {
      type_name: String::from("ForwardedArgs"),
      expression_l: node.expression_l.into(),
    }
  }
}
