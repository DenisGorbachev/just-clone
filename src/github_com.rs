use core::ops::Not;
use errgonomic::{handle, handle_opt};
use octocrab::Octocrab;
use thiserror::Error;
use url::Url;

pub async fn get_repo_url_from_github_com_url(url: &Url) -> Result<Url, GetRepoUrlFromGithubComUrlError> {
    use GetRepoUrlFromGithubComUrlError::*;
    let (owner, name) = handle!(get_owner_name_from_github_url(url), GetOwnerNameFromGithubUrlFailed);
    let octocrab = handle!(Octocrab::builder().build(), OctocrabBuildFailed);
    let repository = handle!(octocrab.repos(owner, name).get().await, ReposGetFailed, owner, name);
    let repo_url = handle_opt!(repository.clone_url, CloneUrlNotFound, owner, name);
    Ok(repo_url)
}

#[derive(Error, Debug)]
pub enum GetRepoUrlFromGithubComUrlError {
    #[error("failed to get owner and name from GitHub URL")]
    GetOwnerNameFromGithubUrlFailed { source: GetOwnerNameFromGithubUrlError },
    #[error("failed to build GitHub client")]
    OctocrabBuildFailed { source: Box<octocrab::Error> },
    #[error("failed to get GitHub repo '{owner}/{name}'")]
    ReposGetFailed { source: Box<octocrab::Error>, owner: String, name: String },
    #[error("GitHub repo '{owner}/{name}' has no clone URL")]
    CloneUrlNotFound { owner: String, name: String },
}

pub fn get_owner_name_from_github_url(url: &Url) -> Result<(&str, &str), GetOwnerNameFromGithubUrlError> {
    use GetOwnerNameFromGithubUrlError::*;
    let mut segments = handle_opt!(url.path_segments(), PathSegmentsNotFound, url: url.clone());
    let owner = handle_opt!(segments.next().filter(|segment| segment.is_empty().not()), OwnerNotFound, url: url.clone());
    let name = handle_opt!(segments.next().map(|segment| segment.trim_end_matches(".git")).filter(|name| name.is_empty().not()), NameNotFound, url: url.clone());
    Ok((owner, name))
}

#[derive(Error, Debug)]
pub enum GetOwnerNameFromGithubUrlError {
    #[error("URL '{url}' has no path segments")]
    PathSegmentsNotFound { url: Url },
    #[error("URL '{url}' has no owner segment")]
    OwnerNotFound { url: Url },
    #[error("URL '{url}' has no name segment")]
    NameNotFound { url: Url },
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn must_return_error_for_reserved_path_segments() {
        todo!()
    }
}
