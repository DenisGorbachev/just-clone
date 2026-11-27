use crate::{GetDir, GetPathError, get_path};
use derive_new::new;
use std::path::PathBuf;
use url::Url;

#[derive(new, Clone, Copy, Debug)]
pub struct GetDirFn<Error> {
    pub inner: fn(&Url) -> Result<PathBuf, Error>,
}

impl Default for GetDirFn<GetPathError> {
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
