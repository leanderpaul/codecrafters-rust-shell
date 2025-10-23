use std::{fs, path::PathBuf};

use crate::state::STATE;

pub fn execute(args: &[&str]) {
  let input = args.get(0).unwrap_or(&"~");
  let mut path: PathBuf = PathBuf::from(input);

  if input == &"~" {
    let home = std::env::var("HOME").unwrap_or_default();
    path = PathBuf::from(home);
  }

  if input.starts_with(".") {
    let cwd = &STATE.read().unwrap().cwd;
    path = PathBuf::from(cwd).join(path);
  }

  match fs::canonicalize(path) {
    Ok(canonical_path) => STATE.write().unwrap().cwd = canonical_path,
    Err(_) => eprintln!("cd: {}: No such file or directory", input),
  }
}
