use url::Url;

use crate::crates_io::get_repo_url_from_crates_io_url;
use crate::github_com::get_repo_url_from_github_com_url;
use crate::types::outcome::Outcome;

pub async fn get_url(url: Url) -> Outcome<Url> {
    match url.domain() {
        Some("crates.io") => get_repo_url_from_crates_io_url(&url)
            .await
            .map_err(From::from),
        Some("docs.rs") => todo!(),
        Some("github.com") => get_repo_url_from_github_com_url(&url).await,
        Some(_) => todo!(),
        None => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use url_macro::url;

    use crate::functions::get_url::get_url;
    use crate::types::outcome::Outcome;

    #[tokio::test]
    async fn must_get_url_for_crates_io() -> Outcome {
        let url = get_url(url!("https://crates.io/crates/url-macro")).await?;
        assert_eq!(url, url!("https://github.com/DenisGorbachev/url-macro"));
        Ok(())
    }
}
