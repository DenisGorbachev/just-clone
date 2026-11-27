use crate::{GetOwnerNameFromGithubUrlError, get_owner_name_from_github_url};
use errgonomic::handle;
use std::path::PathBuf;
use thiserror::Error;
use url::Url;

pub fn get_path(url: &Url) -> Result<PathBuf, GetPathError> {
    use GetPathError::*;
    match url.domain() {
        Some("github.com") => {
            let (owner, name) = handle!(get_owner_name_from_github_url(url), GetOwnerNameFromGithubUrlFailed);
            Ok(PathBuf::from(format!("{owner}•{name}")))
        }
        _ => Err(UnsupportedUrl {
            url: url.clone(),
        }),
    }
}

#[derive(Error, Debug)]
pub enum GetPathError {
    #[error("failed to get owner and name from GitHub URL")]
    GetOwnerNameFromGithubUrlFailed { source: GetOwnerNameFromGithubUrlError },
    #[error("can't derive a dir from URL '{url}'")]
    UnsupportedUrl { url: Url },
}
