use failure::ResultExt;
use std::fs::OpenOptions;
use std::path::Path;

/// A record of changes that have occurred on a project.
pub struct Changelog;

impl Changelog {
  /// Open a file to `CHANGELOG.md`, or create it if it doesn't exist already.
  pub fn open(path: &str) -> ::Result<Self> {
    let path = Path::new(path).join("CHANGELOG.md");
    let _file = OpenOptions::new()
      .append(true)
      .open(path)
      .context(::ErrorKind::Fs)?;
    Ok(Changelog {})
  }
}
