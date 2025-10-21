use std::path::Path;

use crate::state::STATE;

pub fn execute(args: &[&str]) {
  let path = Path::new(args.get(0).unwrap());
  if path.exists() {
    STATE.write().unwrap().cwd = path.to_path_buf();
  } else {
    eprintln!("cd: {}: No such file or directory", path.display());
  }
}
