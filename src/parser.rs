use std::{
  fs::read_to_string,
  io::{stdout, BufWriter, Stdout, Write},
  path::Path
};

use miette::Report;
use oxc_allocator::Allocator;
use oxc_diagnostics::{DiagnosticService, GraphicalReportHandler, OxcDiagnostic};
use oxc_parser::Parser;
use oxc_span::SourceType;

use crate::error::ScanError;

pub struct OxcParser {
  diagnostics: Vec<Report>,
  writer: BufWriter<Stdout>,
}

impl OxcParser {
  pub fn new() -> Self {
    Self {
      diagnostics: vec![],
      writer: BufWriter::new(stdout()),
    }
  }

  pub fn scan_file(&mut self, file: &str) -> Result<(), ScanError> {
    let source_text = read_to_string(file)
      .map_err(|err| ScanError::FileReadError(format!("{}, reading {}", err.to_string(), file)))?;
    let source_type = SourceType::from_path(file).unwrap();
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
      self.collect_diagnostics(file, &source_text, ret.errors);
    }

    Ok(())
  }

  pub fn report(&mut self) {
    let handler = GraphicalReportHandler::new();
    let mut out = String::new();

    for diagnostic in &self.diagnostics {
      let _ = handler.render_report(&mut out, diagnostic.as_ref());
    }

    let _ = self.writer.write_all(out.as_bytes());
    self.diagnostics.clear();
  }

  fn collect_diagnostics(&mut self, file: &str, source_text: &str, errors: Vec<OxcDiagnostic>) {
    let mut diagnostics =
      DiagnosticService::wrap_diagnostics(Path::new(file), &source_text, errors);
    self.diagnostics.append(&mut diagnostics.1);
  }
}
