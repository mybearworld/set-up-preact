use std::fmt::Display;
use std::fs;
use std::process::Command;
mod files;

fn error<T, U>(action: T, err: U)
where
  T: Display,
  U: Display,
{
  eprintln!("Error: Failed to {action}: {err}");
}

fn main() {
  println!("Modifying files...");

  let inter = include_bytes!("../fonts/Inter.var.woff2");
  fs::create_dir("src/assets/fonts").unwrap_or_else(|err| {
    error("create src/assets/fonts", err);
  });
  fs::write("src/assets/fonts/Inter.var.woff2", inter).unwrap_or_else(|err| {
    error("create src/assets/fonts/Inter.var.woff2", err);
  });

  for [path, contents] in files::files() {
    fs::write(path, contents).unwrap_or_else(|err| {
      error(format!("write {path}"), err);
    });
  }

  fs::remove_file("src/assets/preact.svg").unwrap_or_else(|err| {
    error("delete src/assets/preact.svg", err);
  });
  fs::remove_dir_all("public").unwrap_or_else(|err| {
    error("delete public", err);
  });

  let pnpm = which::which("pnpm");

  if let Ok(pnpm) = pnpm {
    println!("Installing dependencies...");

    if let Err(err) = Command::new(&pnpm).arg("install").output() {
      error("install Preact's dependencies", err);
    }

    if let Err(err) = Command::new(&pnpm)
      .arg("install")
      .arg("tailwindcss")
      .arg("postcss")
      .arg("autoprefixer")
      .arg("prettier")
      .arg("prettier-plugin-tailwindcss")
      .output()
    {
      error("install the project's dependencies", err);
    }

    println!("Formatting...");
    if let Err(err) = Command::new(&pnpm)
      .arg("exec")
      .arg("prettier")
      .arg("-w")
      .arg("**/*.*")
      .output()
    {
      error("format", err);
    }
  } else if let Err(err) = pnpm {
    error("find PNPM", err);
  }

  println!("Done! You can begin coding now :)")
}
