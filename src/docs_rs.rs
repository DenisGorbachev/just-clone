use crate::Outcome;
use crate::{get_repo_url_from_crate_name, CrateName};
use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;
use not_found_error::OkOrNotFound;
use url::Url;

pub async fn get_repo_url_from_docs_rs_url(url: &Url) -> Outcome<Url> {
    let crate_name = get_crate_name_from_docs_rs_url(url)?;
    let repo_url = get_repo_url_from_crate_name(crate_name.as_str()).await?;
    Ok(repo_url)
}

pub fn get_crate_name_from_docs_rs_url(url: &Url) -> Outcome<CrateName> {
    let mut segments = url.path_segments().ok_or_not_found::<CrateName>()?;
    let first_segment = segments.next().ok_or_not_found::<CrateName>()?;
    match first_segment {
        "releases" => Err(NotACrateUrl.into()),
        other => {
            let crate_name = CrateName::try_from(other)?;
            Ok(crate_name)
        }
    }
}

// fn find_three_segments<'a>(mut segments: impl Iterator<Item = &'a str>) -> Option<(&'a str, &'a str, &'a str)> {
//     Some((segments.next()?, segments.next()?, segments.next()?))
// }

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NotACrateUrl;

#[cfg(test)]
mod tests {
    use super::*;
    use url_macro::url;

    #[test]
    fn must_get_crate_name() {
        let syn_crate_url = url!("https://docs.rs/syn/latest/syn/struct.ItemImpl.html");
        assert_eq!(get_crate_name_from_docs_rs_url(&syn_crate_url).unwrap(), "syn".try_into().unwrap());

        // NOTE: "dioxus-tui" != "dioxus_tui"
        let dioxus_tui_crate_url = url!("https://docs.rs/dioxus-tui/latest/dioxus_tui/index.html");
        assert_eq!(get_crate_name_from_docs_rs_url(&dioxus_tui_crate_url).unwrap(), "dioxus-tui".try_into().unwrap());

        let releases_internal_url = url!("https://docs.rs/releases");
        assert!(get_crate_name_from_docs_rs_url(&releases_internal_url).is_err())
    }
}
