use lib_ruby_parser::Bytes as InnerBytes;

#[napi(object)]
pub struct Bytes {
  /// Raw vector of bytes
  pub raw: Vec<u8>,
}

impl From<InnerBytes> for Bytes {
  fn from(val: InnerBytes) -> Self {
    Self { 
      raw: val.raw,
     }
  }
}