use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum ScanError {
  FileReadError(String),
  SyntaxError(String),
  InvalidExtension(String),
}

// impl ScanError {
//   pub fn unwrap(self) -> String {
//     match self {
//       Self::FileNotFound(msg) | Self::SyntaxError(msg) | Self::InvalidExtension(msg) => msg,
//     }
//   }
// }
