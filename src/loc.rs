#[napi(object)]
pub struct Loc {
  pub begin: u32,
  pub end: u32,
}

impl From<lib_ruby_parser::Loc> for Loc {
  fn from(loc: lib_ruby_parser::Loc) -> Self {
    Self {
      begin: loc.begin as u32,
      end: loc.end as u32,
    }
  }
}