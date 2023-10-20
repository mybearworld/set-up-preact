use std::fs;
mod files;

fn main() {
  for [path, contents] in files::files() {
    fs::write(path, contents)
      .unwrap_or_else(|err| eprintln!("Note: Failed to write {path}: {err}"));
  }
}
