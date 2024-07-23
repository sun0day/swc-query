#![deny(clippy::all)]

mod error;
mod parser;
use napi::bindgen_prelude::Buffer;
use parser::OxcParser;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Scanner {
  parser: OxcParser,
}

#[napi]
impl Scanner {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      parser: OxcParser::new(),
    }
  }

  #[napi]
  pub fn scan(&mut self, file: Buffer) -> Buffer {
    let file = String::from_utf8_lossy(&file).to_string();

    let parse_result = self.parser.scan_file(&file);

    match parse_result {
      Ok(_) => "".as_bytes().into(),
      Err(err) => serde_json::to_string(&err).unwrap().as_bytes().into(),
    }

    // buffer.to_string().as_bytes().into()
  }

  #[napi]
  pub fn report(&mut self) {
    self.parser.report();
  }
}
