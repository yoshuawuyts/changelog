use failure::ResultExt;
use std::fs::OpenOptions;
use std::path::Path;

/// A record of changes that have occurred on a project.
pub struct Changelog;

impl Changelog {
  /// Open a file to `CHANGELOG.md`.
  pub fn open(path: &str) -> ::Result<Self> {
    let path = Path::new(path).join("CHANGELOG.md");
    let _file = OpenOptions::new()
      .append(true)
      .open(path)
      .context(::ErrorKind::Fs)?;
    Ok(Changelog {})
  }

  /// Create a new changelog at `CHANGELOG.md`.
  pub fn create(path: &str) -> ::Result<Self> {
    let path = Path::new(path).join("CHANGELOG.md");
    let _file = OpenOptions::new()
      .write(true)
      .open(path)
      .context(::ErrorKind::Fs)?;
    Ok(Changelog {})
  }

  /// Open a changelog, or create it if it doesn't exist yet.
  pub fn open_or_create(path: &str) -> ::Result<Self> {
    unimplemented!();
  }
}
