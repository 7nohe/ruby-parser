use lib_ruby_parser::nodes::ForwardedArgs as InnerForwardedArgs;

#[napi(object)]
pub struct ForwardedArgs {

}

impl From<InnerForwardedArgs> for ForwardedArgs {
  fn from(node: InnerForwardedArgs) -> Self {
    Self {

    }
  }
}
