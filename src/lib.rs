#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;
extern crate chrono;
extern crate git2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate mktemp;
extern crate toml;

mod changelog;
mod cli;
mod error;
mod git;

pub use changelog::{format, stats};
pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
pub use git::{all_commits, full_diff, Commit, Tag};

use failure::ResultExt;
use mktemp::Temp;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
  package: Package,
}

#[derive(Deserialize)]
struct Package {
  repository: String,
}

/// Get the repository name from `Cargo.toml`.
pub fn read_repo(dir: &str) -> ::Result<String> {
  let mut dir = PathBuf::from(dir);
  dir.push("Cargo.toml");
  let cargo_toml = fs::read_to_string(dir).context(::ErrorKind::Other)?;
  let config: Config =
    toml::from_str(&cargo_toml).context(::ErrorKind::Other)?;
  Ok(config.package.repository)
}

/// Prepend a changelog to a file.
pub fn prepend_file(file_path: &str, data: &str) -> ::Result<()> {
  let file_path = PathBuf::from(file_path);

  // Touch new file if it doesn't exist already
  let file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&file_path)
    .context(::ErrorKind::Other)?;
  file.sync_all().context(::ErrorKind::Other)?;

  // Setup temp file & path
  let tmp_path = Temp::new_file().context(::ErrorKind::Other)?;
  let mut tmp = File::create(&tmp_path).context(::ErrorKind::Other)?;
  let mut src = File::open(&file_path).context(::ErrorKind::Other)?;

  // Prepend data
  tmp.write_all(data.as_bytes()).context(::ErrorKind::Other)?;
  tmp.write(b"\n\n").context(::ErrorKind::Other)?;
  io::copy(&mut src, &mut tmp).context(::ErrorKind::Other)?;

  // Cleanup
  fs::remove_file(&file_path).context(::ErrorKind::Other)?;
  fs::copy(&tmp_path, &file_path).context(::ErrorKind::Other)?;

  Ok(())
}
