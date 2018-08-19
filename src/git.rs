use failure::ResultExt;
use git2::Repository;

/// A git commit.
#[derive(Clone, Debug)]
pub struct Commit {
  message: String,
  hash: Vec<u8>,
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
  pub fn hash(&self) -> &[u8] {
    &self.hash
  }
}

/// Get all commits for a path.
#[must_use]
pub fn all_commits(path: &str) -> ::Result<Vec<Commit>> {
  let repo = Repository::open(path).context(::ErrorKind::Git)?;
  let mut revwalk = repo.revwalk().context(::ErrorKind::Git)?;
  let head = repo.head().context(::ErrorKind::Git)?;
  let oid = head.target().ok_or(::ErrorKind::Git)?;

  revwalk.push(oid).context(::ErrorKind::Git)?;
  let revwalk = revwalk.filter_map(|id| repo.find_commit(id.ok()?).ok());

  let mut commits = vec![];
  for commit in revwalk {
    let message = commit.message().ok_or(::ErrorKind::Git)?.to_string();
    let hash = commit.id().as_bytes().to_owned();
    commits.push(Commit { message, hash });
  }

  Ok(commits)
}
