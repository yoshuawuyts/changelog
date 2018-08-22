use super::{Commit, Tag};
use std::str;

/// Format a list of commits to a changelog entry
#[must_use]
pub fn format(tag: &Tag, commits: &[Commit], repo_url: &str) -> String {
  let date = commits[0].datetime().naive_utc().date();
  let version = match tag.name() {
    Some(name) => name,
    None => "<unknown version>",
  };

  // Title
  let mut changelog = format!("## {}, Version {}\n", date, version);

  // Commits
  changelog.push_str("### Commits");
  for commit in commits {
    let long_hash = commit.hash();
    let short_hash = truncate(long_hash, 10);
    let url = format!("{}/commits/{}", repo_url, long_hash);
    let hash = format!("[`{}`]({})", short_hash, url);

    let mut msg = commit.message().to_string();
    msg.pop(); // remove trailing newline

    let author = match commit.author() {
      Some(author) => format!("({})", author),
      None => String::from(""),
    };

    let comm = format!("\n- [{}] {} {}", hash, msg, author);
    changelog.push_str(&comm);
  }

  changelog
}

/// Truncate a string to be max `n` chars long.
fn truncate(s: &str, max_chars: usize) -> &str {
  match s.char_indices().nth(max_chars) {
    None => s,
    Some((idx, _)) => &s[..idx],
  }
}

/// Get the stats section.
pub fn stats(diff: &str) -> ::Result<String> {
  Ok(format!("\n\n### Stats\n```diff\n{}```\n", diff))
}
