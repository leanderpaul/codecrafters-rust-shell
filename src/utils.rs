use std::{fs::DirEntry, os::unix::fs::PermissionsExt, path::Path};

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
      if !entry.path().exists() {
        continue;
      }

      let is_file = entry.path().is_file();
      let is_executable = entry.path().metadata().unwrap().permissions().mode() & 0o111 != 0;
      if entry.file_name() == cmd && is_file && is_executable {
        return Some(entry);
      }
    }
  }

  None
}
