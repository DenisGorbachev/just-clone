use std::path::PathBuf;

use url::Url;

use crate::github_com::get_path_from_github_url;
use crate::Outcome;

pub fn get_path(url: &Url) -> Outcome<PathBuf> {
    match url.domain() {
        Some("github.com") => get_path_from_github_url(url),
        Some(_) => todo!(),
        None => todo!(),
    }
}
