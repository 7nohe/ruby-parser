use lib_ruby_parser::nodes::IFlipFlop as InnerIFlipFlop;

#[napi(object)]
pub struct IFlipFlop {

}

impl From<InnerIFlipFlop> for IFlipFlop {
  fn from(node: InnerIFlipFlop) -> Self {
    Self {

    }
  }
}
