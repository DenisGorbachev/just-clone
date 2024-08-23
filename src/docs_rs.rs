use crate::Outcome;
use crate::{get_repo_url_from_crate_name, CrateName};
use not_found_error::{NotFoundError, OkOrNotFound};
use url::Url;

pub async fn get_repo_url_from_docs_rs_url(url: &Url) -> Outcome<Url> {
    let crate_name = get_crate_name_from_docs_rs_url(url)?;
    let repo_url = get_repo_url_from_crate_name(crate_name.as_str()).await?;
    Ok(repo_url)
}

pub fn get_crate_name_from_docs_rs_url(url: &Url) -> Outcome<CrateName> {
    let segments = url.path_segments().ok_or_not_found::<CrateName>()?;
    let (crate_name_1, _crate_version, crate_name_2) = find_three_segments(segments).ok_or_not_found::<CrateName>()?;
    if crate_name_1 != crate_name_2 {
        return NotFoundError::<CrateName>::result();
    }
    let crate_name = CrateName::try_from(crate_name_1)?;
    Ok(crate_name)
}

fn find_three_segments<'a>(mut segments: impl Iterator<Item = &'a str>) -> Option<(&'a str, &'a str, &'a str)> {
    Some((segments.next()?, segments.next()?, segments.next()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use url_macro::url;

    #[test]
    fn must_get_crate_name() {
        let syn_crate_url = url!("https://docs.rs/syn/latest/syn/struct.ItemImpl.html");
        assert_eq!(get_crate_name_from_docs_rs_url(&syn_crate_url).unwrap(), "syn".try_into().unwrap());

        let releases_internal_url = url!("https://docs.rs/releases");
        assert!(get_crate_name_from_docs_rs_url(&releases_internal_url).is_err())
    }
}
