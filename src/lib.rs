#![deny(clippy::all)]

mod error;
mod parser;
mod path;
use napi::bindgen_prelude::Buffer;
use parser::OxcParser;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Scanner {
  parser: OxcParser,
}

fn buffer_to_string(buf: Buffer) -> String {
  String::from_utf8_lossy(&buf).to_string()
}

#[napi]
impl Scanner {
  #[napi(constructor)]
  pub fn new(root: Buffer) -> Self {
    Self {
      parser: OxcParser::new(&buffer_to_string(root)),
    }
  }

  #[napi]
  pub fn scan(&mut self, file: Buffer) -> Buffer {
    let file = buffer_to_string(file);

    let parse_result = self.parser.scan_file(&file);

    match parse_result {
      Ok(_) => "".as_bytes().into(),
      Err(err) => serde_json::to_string(&err).unwrap().as_bytes().into(),
    }
  }

  #[napi]
  pub fn report(&mut self) {
    self.parser.report();
  }
}
