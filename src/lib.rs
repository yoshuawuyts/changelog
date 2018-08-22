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

pub use changelog::format;
pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
pub use git::{all_commits, Commit, Tag};

use failure::ResultExt;
use mktemp::Temp;
use std::fs;
use std::path::PathBuf;
use std::{fs::File, io, io::Write};

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
  let mut tmp_path = Temp::new_file().context(::ErrorKind::Other)?;
  tmp_path.release();
  let mut tmp = File::create(&tmp_path).context(::ErrorKind::Other)?;
  let mut src = File::open(&file_path).context(::ErrorKind::Other)?;
  tmp.write_all(data.as_bytes()).context(::ErrorKind::Other)?;
  io::copy(&mut src, &mut tmp).context(::ErrorKind::Other)?;
  fs::remove_file(&file_path).context(::ErrorKind::Other)?;
  fs::rename(&tmp_path, &file_path).context(::ErrorKind::Other)?;
  Ok(())
}
