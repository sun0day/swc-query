use std::{env::consts::OS, path::Path};

pub fn reverse_backslash(path: &str) -> String {
  if OS == "windows" {
    path.replace(r"\", "/")
  } else {
    path.to_owned()
  }
}

pub fn absolute_path(root: &str, path: &str) -> String {
  let path_tmp = Path::new(path);
  if path_tmp.is_absolute() {
    path.to_owned()
  } else {
    Path::new(root).join(path_tmp).to_string_lossy().to_string()
  }
}

pub fn relative_path(root: &str, path: &str) -> String {
  let path_tmp = Path::new(path);
  match path_tmp.strip_prefix(root) {
    Ok(p) => p.to_string_lossy().to_string(),
    Err(_) => path.to_owned(),
  }
}
