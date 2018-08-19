use failure::ResultExt;
use git2::Repository;

pub fn git(path: &str) -> ::Result<()> {
  let repo = Repository::open(path).context(::ErrorKind::Git)?;
  let refr = repo.head().context(::ErrorKind::Git)?;
  println!("{:?}", refr.name());
  Ok(())
}
