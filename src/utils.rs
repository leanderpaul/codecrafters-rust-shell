use std::{fs::DirEntry, path::Path};

pub fn find_command_in_path(cmd: &str) -> Option<DirEntry> {
  let path = std::env::var("PATH").unwrap_or_default();
  let paths: Vec<&str> = path.split(':').collect();

  for p in paths {
    let path = Path::new(p);
    if !path.is_dir() {
      continue;
    }

    for entry in path.read_dir().unwrap() {
      let entry = entry.unwrap();
      if entry.file_name() == cmd {
        return Some(entry);
      }
    }
  }

  None
}
