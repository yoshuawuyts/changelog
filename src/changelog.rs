use super::{Commit, Tag};

/// Format a list of commits to a changelog entry
pub fn format(tag: &Tag, commits: &[Commit]) -> String {
  println!("tag {:?}", tag);
  for commit in commits {
    println!("commit {:?}", commit);
  }
  unimplemented!();
}
