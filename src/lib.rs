#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
mod napi_parser_result;
mod napi_box;
mod types;
mod bytes;
mod loc;

use lib_ruby_parser::{Parser, ParserOptions};
use napi_parser_result::{from_node, RubyNode};

#[napi]
pub fn parse(text: String) -> napi::Result<RubyNode> {

  let options = ParserOptions {
    buffer_name: "(eval)".to_string(),
    ..Default::default()
  };
  let parser = Parser::new(text.as_bytes().to_vec(), options);
  let result = parser.do_parse();

  let ast = result.ast;

  match ast {
    Some(node) => {
      let ruby_node = from_node(*node);
      Ok(ruby_node)
    },
    None => {
      Err(napi::Error::new(napi::Status::GenericFailure, "Failed to parse"))
    }
  }
}
