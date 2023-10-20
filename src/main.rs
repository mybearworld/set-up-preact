use std::fs;
use std::process::Command;
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

  if let Ok(pnpm) = which::which("pnpm") {
    println!("Installing dependencies...");

    if let Err(err) = Command::new(&pnpm).arg("install").output() {
      eprintln!("Note: Failed to install Preact's dependencies: {err}");
    }

    if let Err(err) = Command::new(&pnpm)
      .arg("install")
      .arg("tailwindcss")
      .arg("postcss")
      .arg("autoprefixer")
      .arg("prettier")
      .output()
    {
      eprintln!("Note: Failed to the project's dependencies: {err}");
    }

    println!("Formatting...");
    if let Err(err) = Command::new(&pnpm)
      .arg("exec")
      .arg("prettier")
      .arg("-w")
      .arg("**/*.*")
      .output()
    {
      eprintln!("Note: Failed to format: {err}")
    }
  } else {
    eprintln!("Note: Couldn't find PNPM.");
  }

  println!("Done! You can begin coding now :)")
}
