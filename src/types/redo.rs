use lib_ruby_parser::nodes::Redo as InnerRedo;

#[napi(object)]
pub struct Redo {

}

impl From<InnerRedo> for Redo {
  fn from(node: InnerRedo) -> Self {
    Self {

    }
  }
}
