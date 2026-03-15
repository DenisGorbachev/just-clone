use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;
use url::Url;

use crate::Outcome;
use crate::crates_io::get_repo_url_from_crates_io_url;
use crate::get_repo_url_from_docs_rs_url;
use crate::github_com::get_repo_url_from_github_com_url;

pub const MAX_URL_RESOLUTION_ITERATIONS: usize = 10;

pub async fn get_url(url: Url) -> Outcome<Url> {
    let mut url = url;
    let mut iterations = 0..MAX_URL_RESOLUTION_ITERATIONS;
    while iterations.next().is_some() {
        match url.domain() {
            Some("crates.io") => {
                url = get_repo_url_from_crates_io_url(&url).await?;
            }
            Some("docs.rs") => {
                url = get_repo_url_from_docs_rs_url(&url).await?;
            }
            Some("github.com") => return get_repo_url_from_github_com_url(&url).await,
            Some(_) => return Ok(url),
            None => return Ok(url),
        }
    }

    Err(MaxUrlResolutionIterationsExceededError::new(url).into())
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct MaxUrlResolutionIterationsExceededError {
    url: Url,
}

#[cfg(test)]
mod tests {
    use url_macro::url;

    use crate::Outcome;
    use crate::functions::get_url::get_url;

    #[tokio::test]
    async fn must_get_url_for_crates_io() -> Outcome {
        let url = get_url(url!("https://crates.io/crates/url-macro")).await?;
        assert_eq!(url, url!("https://github.com/DenisGorbachev/url-macro.git"));
        Ok(())
    }
}
