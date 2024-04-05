use lib_ruby_parser::nodes::File as InnerFile;
use crate::loc::Loc;

#[napi(object)]
pub struct File {
  #[napi(ts_type = "'File'")]
  pub type_name: String,
  pub expression_l: Loc,
}

impl From<InnerFile> for File {
  fn from(node: InnerFile) -> Self {
    Self {
      type_name: String::from("File"),
      expression_l: node.expression_l.into(),
    }
  }
}