use crate::Outcome;
use crate::{NameNotFoundError, OwnerNotFoundError, PathNotFoundError};
use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;
use octocrab::Octocrab;
use std::path::PathBuf;
use url::Url;

pub async fn get_repo_url_from_github_com_url(url: &Url) -> Outcome<Url> {
    let (owner, name) = get_owner_name_from_github_url(url)?;
    let octocrab = Octocrab::builder().build()?;
    let repository = octocrab.repos(owner, name).get().await?;
    let clone_url = repository.clone_url.ok_or(GithubCloneUrlNotFoundError)?;
    Ok(clone_url)
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GithubCloneUrlNotFoundError;

pub fn get_path_from_github_url(url: &Url) -> Outcome<PathBuf> {
    let (owner, name) = get_owner_name_from_github_url(url)?;
    let path_buf = [owner, name].iter().collect();
    Ok(path_buf)
}

pub fn get_owner_name_from_github_url(url: &Url) -> Outcome<(&str, &str)> {
    let mut segments = url.path_segments().ok_or(PathNotFoundError)?;
    let owner = segments.next().ok_or(OwnerNotFoundError)?;
    let name = segments.next().ok_or(NameNotFoundError)?;
    let name = name.trim_end_matches(".git");
    Ok((owner, name))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn must_return_error_for_reserved_path_segments() {
        todo!()
    }
}
