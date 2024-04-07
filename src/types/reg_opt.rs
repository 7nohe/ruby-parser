use lib_ruby_parser::nodes::RegOpt as InnerRegOpt;

#[napi(object)]
pub struct RegOpt {

}

impl From<InnerRegOpt> for RegOpt {
  fn from(node: InnerRegOpt) -> Self {
    Self {

    }
  }

}