use chrono::prelude::*;
use failure::ResultExt;
use git2::{self, DiffStatsFormat, Repository};
use std::str;

/// A git tag.
#[derive(Clone, Debug)]
pub struct Tag {
  name: Option<String>,
}

impl Tag {
  /// Access the tag name.
  #[inline]
  #[must_use]
  pub fn name(&self) -> &Option<String> {
    &self.name
  }
}

/// A git commit.
#[derive(Clone, Debug)]
pub struct Commit {
  message: String,
  hash: String,
  author: Option<String>,
  datetime: DateTime<Utc>,
}

impl Commit {
  /// Access the commit message.
  #[inline]
  #[must_use]
  pub fn message(&self) -> &str {
    &self.message
  }

  /// Access the commit hash.
  #[inline]
  #[must_use]
  pub fn hash(&self) -> &str {
    &self.hash
  }

  /// Access the commit author.
  #[inline]
  #[must_use]
  pub fn author(&self) -> &Option<String> {
    &self.author
  }

  /// Access the commit datetime.
  #[inline]
  #[must_use]
  pub fn datetime(&self) -> &DateTime<Utc> {
    &self.datetime
  }
}

/// Diff two git objects.
pub fn diff(
  repo: &Repository,
  o1: git2::Object,
  o2: git2::Object,
) -> ::Result<String> {
  let t1 = o1.peel_to_tree().context(::ErrorKind::Git)?;
  let t2 = o2.peel_to_tree().context(::ErrorKind::Git)?;
  let diff = repo
    .diff_tree_to_tree(Some(&t2), Some(&t1), None)
    .context(::ErrorKind::Git)?;
  let stats = diff.stats().context(::ErrorKind::Git)?;
  let format = DiffStatsFormat::FULL;
  let buf = stats.to_buf(format, 80).context(::ErrorKind::Git)?;
  let buf = str::from_utf8(&*buf).context(::ErrorKind::Other)?;
  Ok(buf.to_owned())
}

/// Get the latest two commits for the range.
#[must_use]
pub fn get_commit_range(
  repo: &Repository,
) -> ::Result<(git2::Object, git2::Object)> {
  let tags = repo.tag_names(None).context(::ErrorKind::Git)?;
  let len = tags.len();

  let (start, end) = match len {
    0 => return Err(::ErrorKind::NoTags.into()),
    1 => (tags.get(len - 1), None),
    _ => (tags.get(len - 1), tags.get(len - 2)),
  };

  // Value has to be `Some()` here.
  let start = start.expect("Tag should have a value.");
  let (start, end) = match (start, end) {
    (start, None) => {
      let start = repo.revparse_single(start).context(::ErrorKind::Git)?;
      let mut revwalk = repo.revwalk().context(::ErrorKind::Git)?;
      revwalk.push(start.id()).context(::ErrorKind::Git)?;
      revwalk.set_sorting(git2::Sort::REVERSE);
      let oid = revwalk
        .nth(0)
        .ok_or(::ErrorKind::Git)?
        .context(::ErrorKind::Git)?;
      let last = repo.find_object(oid, None).unwrap();
      (start, last)
    }
    (start, Some(end)) => (
      repo.revparse_single(start).context(::ErrorKind::Git)?,
      repo.revparse_single(end).context(::ErrorKind::Git)?,
    ),
  };

  Ok((start, end))
}

/// Get the full diff in a single convenience function.
pub fn full_diff(path: &str) -> ::Result<String> {
  let repo = Repository::open(path).context(::ErrorKind::Git)?;
  let (start, end) = get_commit_range(&repo)?;
  Ok(diff(&repo, start, end)?)
}

/// Get all commits for a path.
#[must_use]
pub fn all_commits(path: &str) -> ::Result<(Tag, Vec<Commit>)> {
  let repo = Repository::open(path).context(::ErrorKind::Git)?;
  let (start, end) = get_commit_range(&repo)?;

  let tag = match start.as_tag() {
    None => unreachable!(),
    Some(tag) => Tag {
      name: tag.name().map(|tag| tag.to_owned()),
    },
  };

  let mut revwalk = repo.revwalk().context(::ErrorKind::Git)?;
  revwalk.push(start.id()).context(::ErrorKind::Git)?;
  let revwalk = revwalk.filter_map(|id| repo.find_commit(id.ok()?).ok());

  let mut commits = vec![];
  for commit in revwalk {
    let end = end.as_tag().expect("Object should have been a tag");
    if end.target_id() == commit.id() {
      break;
    }
    let message = commit.message().ok_or(::ErrorKind::Git)?.to_string();
    let hash = format!("{}", commit.id());
    let author = commit.author().name().map(|name| name.to_owned());
    let timestamp = commit.time().seconds();
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    commits.push(Commit {
      message,
      hash,
      author,
      datetime,
    });
  }

  Ok((tag, commits))
}
