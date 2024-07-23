use std::{fs::read_to_string};

use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_diagnostics::{ NamedSource, Severity, GraphicalReportHandler};

use crate::error::ScanError;

pub struct OxcParser {
  allocator: Allocator,
}

impl OxcParser {
  pub fn new() -> Self {
    Self {
      allocator: Allocator::default()
    }
  }

  pub fn scan_file(&self, file: &str) -> Result<(), ScanError> {
    let source_text = read_to_string(file).map_err(|err| {
      ScanError::FileReadError(format!("{}, reading {}", err.to_string(), file))
    })?;
    let source_type = SourceType::from_path(file).unwrap();
    let ret = Parser::new(&self.allocator, &source_text, source_type).parse();
    let handler = GraphicalReportHandler::new();
    let mut out = String::new();

    if !ret.errors.is_empty() {
      for error in ret.errors {
        let err = error.with_source_code(NamedSource::new("__test__/fixtures/a.js", source_text.clone()));
        let _ = handler.render_report(&mut out, err.as_ref());
      }
    }

    println!("{}", out);

    Ok(())
  }
}