use clap_flags;
use failure::ResultExt;
use structopt::StructOpt;

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Cli {
  #[structopt(flatten)]
  logger: clap_flags::Log,
  /// Project directory
  #[structopt(default_value = ".")]
  path: String,
  /// Write output to file
  #[structopt(short = "o", long = "out")]
  file: Option<String>,
}

impl Cli {
  /// Initialize a logger.
  #[inline]
  pub fn log(&self, name: &str) -> crate::Result<()> {
    self
      .logger
      .start(name)
      .map_err(failure::Error::from_boxed_compat)
      .context(crate::ErrorKind::Log)?;
    Ok(())
  }

  /// Access the path.
  #[inline]
  pub fn path(&self) -> &str {
    &self.path
  }

  /// Access the outfile.
  #[inline]
  pub fn file(&self) -> &Option<String> {
    &self.file
  }
}
