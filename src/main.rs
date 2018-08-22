#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate changelog;
extern crate exitfailure;
extern crate structopt;

use changelog::Cli;
use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  let path = args.path();
  let (tag, commits) = changelog::all_commits(&path)?;
  let msg = changelog::format(&tag, &commits);
  println!("{}", msg);
  Ok(())
}
