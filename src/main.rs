use std::fs;
mod files;

fn main() {
  let inter = include_bytes!("../fonts/Inter.var.woff2");
  fs::create_dir("src/assets/fonts")
    .unwrap_or_else(|err| eprintln!("Note: Failed to create src/assets/fonts: {err}"));
  fs::write("src/assets/fonts/Inter.var.woff2", inter).unwrap_or_else(|err| {
    eprintln!("Note: Failed to create src/assets/fonts/Inter.var.woff2: {err}")
  });

  for [path, contents] in files::files() {
    fs::write(path, contents)
      .unwrap_or_else(|err| eprintln!("Note: Failed to write {path}: {err}"));
  }
}
