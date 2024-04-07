use lib_ruby_parser::nodes::Procarg0 as InnerProcarg0;

#[napi(object)]
pub struct Procarg0 {

}

impl From<InnerProcarg0> for Procarg0 {
  fn from(node: InnerProcarg0) -> Self {
    Self {

    }
  }
}
