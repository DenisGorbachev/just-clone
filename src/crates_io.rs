use crates_io_api::AsyncClient;
use derive_more::{Display, Error, From};
use derive_new::new;
use url::Url;

use crate::crates_io::GetRepoUrlFromCrateNameError::RepositoryNotSpecified;

pub async fn get_repo_url_from_crates_io_url(url: &Url) -> Result<Url, GetRepoUrlFromCratesIoUrlError> {
    let crate_name = get_crate_name_from_url(url)?;
    // It should be safe to call .expect() on the next line
    let client = AsyncClient::new("just-clone", std::time::Duration::from_millis(1000)).expect("All headers must be valid");
    let repository_field = get_repo_url_from_crate_name(crate_name, &client).await?;
    let url = parse_url(&repository_field)?;
    Ok(url)
}

// pub fn get_repo_url_from_crate_name_static(name: &str) -> Result<String, GetRepoUrlFromCrateNameError> {
//     get_repo_url_from_crate_name(name, &CRATES_IO_CLIENT)
// }

pub async fn get_repo_url_from_crate_name(name: &str, client: &AsyncClient) -> Result<String, GetRepoUrlFromCrateNameError> {
    let crate_info = client.get_crate(name).await?;
    crate_info
        .crate_data
        .repository
        .ok_or(RepositoryNotSpecified)
}

pub fn parse_url(str: &str) -> Result<Url, url::ParseError> {
    Url::parse(str)
}

pub fn get_crate_name_from_url(url: &Url) -> Result<&str, GetCrateNameFromUrlError> {
    url.path_segments()
        .and_then(find_crate_name_in_path_segments)
        .ok_or_else(|| GetCrateNameFromUrlError {
            url: url.clone(),
        })
}

pub fn find_crate_name_in_path_segments<'a>(mut segments: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    if segments.next() == Some("crates") {
        segments.next()
    } else {
        None
    }
}

#[derive(new, Error, Display, From, Debug)]
pub enum GetRepoUrlFromCratesIoUrlError {
    TheGetCrateNameFromUrlError(GetCrateNameFromUrlError),
    TheGetRepoUrlFromCrateNameError(GetRepoUrlFromCrateNameError),
    TheUrlParseError(url::ParseError),
}

#[derive(new, Error, Display, From, Debug)]
pub enum GetRepoUrlFromCrateNameError {
    RepositoryNotSpecified,
    CratesIoApiError(crates_io_api::Error),
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GetCrateNameFromUrlError {
    url: Url,
}
