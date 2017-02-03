use repository::Repo;
use git2::Repository;
use error::GitError;
pub fn clone(repo: Repo) -> Result<(), GitError> {
    let folder = format!("./.cache/{}", repo.name);
    Repository::clone(repo.clone_url.as_str(), folder)?;
    println!("finished: {}", repo.name);
    Ok(())
}