use crate::functions::get_path::get_path;
use crate::traits::get_dir::GetDir;
use derive_new::new;
use std::path::PathBuf;
use url::Url;

#[derive(new, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct GetDirFn<Error> {
    pub inner: fn(&Url) -> Result<PathBuf, Error>,
}

impl Default for GetDirFn<helpful::Error> {
    fn default() -> Self {
        Self {
            inner: get_path,
        }
    }
}

impl<E> GetDir for fn(&Url) -> Result<PathBuf, E> {
    type Error = E;

    async fn get_dir(self, url: &Url) -> Result<PathBuf, Self::Error> {
        self(url)
    }
}
