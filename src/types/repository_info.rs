use derive_getters::{Dissolve, Getters};
use derive_new::new;
use url::Url;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RepositoryInfo {
    url: Url,
    owner: String,
}

impl RepositoryInfo {}
