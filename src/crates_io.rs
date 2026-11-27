use crates_io_api::{AsyncClient, Error as CratesIoApiError};
use errgonomic::{handle, map_err, map_none};
use std::time::Duration;
use thiserror::Error;
use url::Url;

pub async fn get_repo_url_from_crates_io_url(url: &Url) -> Result<Url, GetRepoUrlFromCratesIoUrlError> {
    use GetRepoUrlFromCratesIoUrlError::*;
    let crate_name = handle!(get_crate_name_from_crates_io_url(url), GetCrateNameFromCratesIoUrlFailed);
    map_err!(get_repo_url_from_crate_name(crate_name).await, GetRepoUrlFromCrateNameFailed)
}

#[derive(Error, Debug)]
pub enum GetRepoUrlFromCratesIoUrlError {
    #[error("failed to get crate name from crates.io URL")]
    GetCrateNameFromCratesIoUrlFailed { source: GetCrateNameFromCratesIoUrlError },
    #[error("failed to get repo URL from crate name")]
    GetRepoUrlFromCrateNameFailed { source: GetRepoUrlFromCrateNameError },
}

pub async fn get_repo_url_from_crate_name(crate_name: &str) -> Result<Url, GetRepoUrlFromCrateNameError> {
    use GetRepoUrlFromCrateNameError::*;
    let client = AsyncClient::new("just-clone", Duration::from_millis(1000)).expect("always succeeds because \"just-clone\" is a valid header value");
    let repository = handle!(get_repo_url_from_crate_name_and_client(crate_name, &client).await, GetRepoUrlFromCrateNameAndClientFailed);
    map_err!(Url::parse(&repository), UrlParseFailed, crate_name, repository)
}

#[derive(Error, Debug)]
pub enum GetRepoUrlFromCrateNameError {
    #[error("failed to get repository of crate")]
    GetRepoUrlFromCrateNameAndClientFailed { source: GetRepoUrlFromCrateNameAndClientError },
    #[error("failed to parse repository URL '{repository}' of crate '{crate_name}'")]
    UrlParseFailed { source: url::ParseError, crate_name: String, repository: String },
}

pub async fn get_repo_url_from_crate_name_and_client(crate_name: &str, client: &AsyncClient) -> Result<String, GetRepoUrlFromCrateNameAndClientError> {
    use GetRepoUrlFromCrateNameAndClientError::*;
    let crate_info = handle!(client.get_crate(crate_name).await, GetCrateFailed, crate_name);
    map_none!(crate_info.crate_data.repository, RepositoryNotSpecified, crate_name)
}

#[derive(Error, Debug)]
pub enum GetRepoUrlFromCrateNameAndClientError {
    #[error("failed to get crate '{crate_name}' from crates.io")]
    GetCrateFailed { source: CratesIoApiError, crate_name: String },
    #[error("crate '{crate_name}' doesn't specify a repository")]
    RepositoryNotSpecified { crate_name: String },
}

pub fn get_crate_name_from_crates_io_url(url: &Url) -> Result<&str, GetCrateNameFromCratesIoUrlError> {
    use GetCrateNameFromCratesIoUrlError::*;
    map_none!(url.path_segments().and_then(find_crate_name_in_path_segments), CrateNameNotFound, url: url.clone())
}

#[derive(Error, Debug)]
pub enum GetCrateNameFromCratesIoUrlError {
    #[error("URL '{url}' has no crate name")]
    CrateNameNotFound { url: Url },
}

pub fn find_crate_name_in_path_segments<'a>(mut segments: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    if segments.next() == Some("crates") { segments.next() } else { None }
}
