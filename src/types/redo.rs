use lib_ruby_parser::nodes::Redo as InnerRedo;

#[napi(object)]
pub struct Redo {
  #[napi(ts_type = "'Redo'")]
  pub type_name: String,
}

impl From<InnerRedo> for Redo {
  fn from(node: InnerRedo) -> Self {
    Self {
      type_name: String::from("Redo"),
    }
  }
}
