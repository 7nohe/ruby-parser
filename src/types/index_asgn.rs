use lib_ruby_parser::nodes::IndexAsgn as InnerIndexAsgn;

#[napi(object)]
pub struct IndexAsgn {

}

impl From<InnerIndexAsgn> for IndexAsgn {
  fn from(node: InnerIndexAsgn) -> Self {
    Self {

    }
  }
}
