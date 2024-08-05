use crates_io_api::SyncClient;
use derive_more::{Display, Error, From};
use derive_new::new;
use lazy_static::lazy_static;
use url::Url;

use crate::crates_io::GetRepoUrlFromCrateNameError::RepositoryNotSpecified;

lazy_static! {
    pub static ref CRATES_IO_CLIENT: SyncClient = SyncClient::new("just-clone", std::time::Duration::from_millis(1000)).unwrap();
}

pub fn get_repo_url_from_crates_io_url(url: &Url) -> Result<Url, GetRepoUrlFromCratesIoUrlError> {
    let crate_name = get_crate_name_from_url(url)?;
    let repository_field = get_repo_url_from_crate_name_static(crate_name)?;
    let url = parse_url(&repository_field)?;
    Ok(url)
}

pub fn get_repo_url_from_crate_name_static(name: &str) -> Result<String, GetRepoUrlFromCrateNameError> {
    get_repo_url_from_crate_name(name, &CRATES_IO_CLIENT)
}

pub fn get_repo_url_from_crate_name(name: &str, client: &SyncClient) -> Result<String, GetRepoUrlFromCrateNameError> {
    let crate_info = client.get_crate(name)?;
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
