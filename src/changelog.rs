use super::{Commit, Tag};

/// Format a list of commits to a changelog entry
#[must_use]
pub fn format(tag: &Tag, commits: &[Commit]) -> String {
  let date = commits[0].datetime().naive_utc().date();
  let version = match tag.name() {
    Some(name) => name,
    None => "<unknown version>",
  };

  // Title
  let mut changelog = format!("## {}, Version {}\n", date, version);

  // Commits
  changelog.push_str("### Commits\n");
  for commit in commits {
    let hash = truncate(commit.hash(), 10);
    let msg = commit.message();
    let comm = format!("- [{}] {}", hash, msg);
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
