use std::io::Error;

use oxc_diagnostics::OxcDiagnostic;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum ScanError {
  FileReadError(String),
  SyntaxError(String),
  InvalidExtension(String),
}

impl ToString for ScanError {
  fn to_string(&self) -> String {
    match self {
      Self::FileReadError(msg) | Self::SyntaxError(msg) | Self::InvalidExtension(msg) => {
        msg.to_string()
      }
    }
  }
}

impl ScanError {
  pub(crate) fn from_file_read(err: &Error, file: &str) -> Self {
    Self::FileReadError(format!("{}, reading {}", err, file))
  }

  pub(crate) fn from_syntax_parse(err: &OxcDiagnostic, file: &str) -> Self {
    Self::SyntaxError(format!("{}, parsing {}", err, file))
  }
}
