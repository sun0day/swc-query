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
    let relative_file = relative_path(&self.root, &file);
    let source_text = read_to_string(&absolute_path(&self.root, &file))
      .map_err(|err| ScanError::from_file_read(&err, &relative_file));

    let source_text = match source_text {
      Ok(text) => text,
      Err(err) => {
        self.collect_diagnostics(
          &relative_file,
          "",
          vec![OxcDiagnostic::error(err.to_string())],
        );
        return Err(err);
      }
    };

    let source_type = SourceType::from_path(&file).unwrap();
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
      let err = Err(ScanError::from_syntax_parse(&ret.errors[0], &file));
      self.collect_diagnostics(&relative_file, &source_text, ret.errors);

      return err;
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
