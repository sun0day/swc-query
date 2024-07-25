use std::{
  fs::read_to_string,
  io::{stdout, BufWriter, Stdout, Write},
  path::Path,
};

use miette::{LabeledSpan, Report};
use oxc_allocator::Allocator;
use oxc_diagnostics::{DiagnosticService, GraphicalReportHandler, OxcDiagnostic};
use oxc_parser::Parser;
use oxc_span::SourceType;

use crate::error::ScanError;
use crate::path::{absolute_path, relative_path, reverse_backslash};

pub struct OxcParser {
  root: String,
  diagnostics: Vec<Report>,
  writer: BufWriter<Stdout>,
}

impl OxcParser {
  pub fn new(root: &str) -> Self {
    Self {
      root: reverse_backslash(root),
      diagnostics: vec![],
      writer: BufWriter::new(stdout()),
    }
  }

  pub fn scan_file(&mut self, file: &str) -> Result<(), ScanError> {
    let file = reverse_backslash(file);
    let source_text = read_to_string(&absolute_path(&self.root, &file))
      .map_err(|err| ScanError::FileReadError(format!("{}, reading {}", err.to_string(), file)))?;
    let source_type = SourceType::from_path(&file).unwrap();
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();
    // println!("-----> s{}", relative_path(&self.root, &file));
    if !ret.errors.is_empty() {
      self.collect_diagnostics(&relative_path(&self.root, &file), &source_text, ret.errors);
    }

    Ok(())
  }

  pub fn report(&mut self) {
    let handler = GraphicalReportHandler::new();
    let mut out = String::new();

    for diagnostic in &self.diagnostics {
      let _ = handler.render_report(&mut out, diagnostic.as_ref());
    }

    self.writer.write_all(out.as_bytes()).unwrap();
    self.diagnostics.clear();
  }

  fn collect_diagnostics(&mut self, file: &str, source_text: &str, errors: Vec<OxcDiagnostic>) {
    let mut diagnostics =
    DiagnosticService::wrap_diagnostics(Path::new(file), &source_text, errors);
    // println!("{:?}", &diagnostics);
    self.diagnostics.append(&mut diagnostics.1);
  }
}
