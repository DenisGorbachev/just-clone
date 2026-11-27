use crate::{CrateName, GetRepoUrlFromCrateNameError, get_repo_url_from_crate_name};
use core::ops::Not;
use errgonomic::{handle, handle_bool, handle_opt, map_err};
use thiserror::Error;
use url::Url;

pub async fn get_repo_url_from_docs_rs_url(url: &Url) -> Result<Url, GetRepoUrlFromDocsRsUrlError> {
    use GetRepoUrlFromDocsRsUrlError::*;
    let crate_name = handle!(get_crate_name_from_docs_rs_url(url), GetCrateNameFromDocsRsUrlFailed);
    map_err!(get_repo_url_from_crate_name(crate_name.as_str()).await, GetRepoUrlFromCrateNameFailed)
}

#[derive(Error, Debug)]
pub enum GetRepoUrlFromDocsRsUrlError {
    #[error("failed to get crate name from docs.rs URL")]
    GetCrateNameFromDocsRsUrlFailed { source: GetCrateNameFromDocsRsUrlError },
    #[error("failed to get repo URL from crate name")]
    GetRepoUrlFromCrateNameFailed { source: GetRepoUrlFromCrateNameError },
}

pub fn get_crate_name_from_docs_rs_url(url: &Url) -> Result<CrateName, GetCrateNameFromDocsRsUrlError> {
    use GetCrateNameFromDocsRsUrlError::*;
    let mut segments = handle_opt!(url.path_segments(), PathSegmentsNotFound, url: url.clone());
    let first_segment = handle_opt!(segments.next().filter(|segment| segment.is_empty().not()), FirstSegmentNotFound, url: url.clone());
    handle_bool!(first_segment == "releases", NotACrateUrl, url: url.clone());
    map_err!(CrateName::try_from(first_segment), CrateNameTryFromFailed, url: url.clone())
}

#[derive(Error, Debug)]
pub enum GetCrateNameFromDocsRsUrlError {
    #[error("URL '{url}' has no path segments")]
    PathSegmentsNotFound { url: Url },
    #[error("URL '{url}' has no first path segment")]
    FirstSegmentNotFound { url: Url },
    #[error("URL '{url}' is not a crate URL")]
    NotACrateUrl { url: Url },
    #[error("failed to create crate name from URL '{url}'")]
    CrateNameTryFromFailed { source: <CrateName as TryFrom<&'static str>>::Error, url: Url },
}

#[cfg(test)]
mod tests {
    use super::*;
    use url_macro::url;

    #[test]
    fn must_get_crate_name() -> Result<(), MustGetCrateNameError> {
        use MustGetCrateNameError::*;
        let syn_crate_url = url!("https://docs.rs/syn/latest/syn/struct.ItemImpl.html");
        let syn_crate_name = handle!(get_crate_name_from_docs_rs_url(&syn_crate_url), GetCrateNameFromDocsRsUrlFailed);
        assert_eq!(syn_crate_name.as_str(), "syn");

        // NOTE: "dioxus-tui" != "dioxus_tui"
        let dioxus_tui_crate_url = url!("https://docs.rs/dioxus-tui/latest/dioxus_tui/index.html");
        let dioxus_tui_crate_name = handle!(get_crate_name_from_docs_rs_url(&dioxus_tui_crate_url), GetCrateNameFromDocsRsUrlFailed);
        assert_eq!(dioxus_tui_crate_name.as_str(), "dioxus-tui");

        let releases_internal_url = url!("https://docs.rs/releases");
        assert!(get_crate_name_from_docs_rs_url(&releases_internal_url).is_err());
        Ok(())
    }

    #[derive(Error, Debug)]
    pub enum MustGetCrateNameError {
        #[error("failed to get crate name from docs.rs URL")]
        GetCrateNameFromDocsRsUrlFailed { source: GetCrateNameFromDocsRsUrlError },
    }
}
