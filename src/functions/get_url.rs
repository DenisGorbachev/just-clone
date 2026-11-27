use crate::{GetRepoUrlFromCratesIoUrlError, GetRepoUrlFromDocsRsUrlError, GetRepoUrlFromGithubComUrlError, get_repo_url_from_crates_io_url, get_repo_url_from_docs_rs_url, get_repo_url_from_github_com_url};
use errgonomic::{handle, map_err};
use thiserror::Error;
use url::Url;

pub const MAX_URL_RESOLUTION_ITERATIONS: usize = 10;

pub async fn get_url(url: Url) -> Result<Url, GetUrlError> {
    use GetUrlError::*;
    let mut url = url;
    let mut iterations = 0..MAX_URL_RESOLUTION_ITERATIONS;
    while iterations.next().is_some() {
        match url.domain() {
            Some("crates.io") => {
                url = handle!(get_repo_url_from_crates_io_url(&url).await, GetRepoUrlFromCratesIoUrlFailed);
            }
            Some("docs.rs") => {
                url = handle!(get_repo_url_from_docs_rs_url(&url).await, GetRepoUrlFromDocsRsUrlFailed);
            }
            Some("github.com") => return map_err!(get_repo_url_from_github_com_url(&url).await, GetRepoUrlFromGithubComUrlFailed),
            Some(_) => return Ok(url),
            None => return Ok(url),
        }
    }

    Err(MaxUrlResolutionIterationsExceeded {
        url,
    })
}

#[derive(Error, Debug)]
pub enum GetUrlError {
    #[error("failed to resolve crates.io URL")]
    GetRepoUrlFromCratesIoUrlFailed { source: GetRepoUrlFromCratesIoUrlError },
    #[error("failed to resolve docs.rs URL")]
    GetRepoUrlFromDocsRsUrlFailed { source: GetRepoUrlFromDocsRsUrlError },
    #[error("failed to resolve GitHub URL")]
    GetRepoUrlFromGithubComUrlFailed { source: GetRepoUrlFromGithubComUrlError },
    #[error("exceeded {max} URL resolution iterations at '{url}'", max = MAX_URL_RESOLUTION_ITERATIONS)]
    MaxUrlResolutionIterationsExceeded { url: Url },
}

#[cfg(test)]
mod integration_tests {
    use crate::{GetUrlError, get_url};
    use errgonomic::handle;
    use thiserror::Error;
    use url_macro::url;

    #[tokio::test]
    async fn must_get_url_for_crates_io() -> Result<(), MustGetUrlForCratesIoError> {
        use MustGetUrlForCratesIoError::*;
        let url = handle!(get_url(url!("https://crates.io/crates/url-macro")).await, GetUrlFailed);
        assert_eq!(url, url!("https://github.com/DenisGorbachev/url-macro.git"));
        Ok(())
    }

    #[derive(Error, Debug)]
    pub enum MustGetUrlForCratesIoError {
        #[error("failed to get URL")]
        GetUrlFailed { source: GetUrlError },
    }
}
