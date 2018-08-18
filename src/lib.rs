#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;
extern crate git2;

mod cli;
mod error;
mod git;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
pub use git::git;
