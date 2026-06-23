use crate::get_path;
use helpful::Error as HelpfulError;
use std::path::PathBuf;
use url::Url;

pub trait GetDir {
    type Error;

    fn get_dir(self, url: &Url) -> impl Future<Output = Result<PathBuf, Self::Error>> + Send;
}

impl GetDir for PathBuf {
    type Error = ();

    async fn get_dir(self, _url: &Url) -> Result<PathBuf, Self::Error> {
        Ok(self)
    }
}

impl GetDir for Option<PathBuf> {
    type Error = HelpfulError;

    async fn get_dir(self, url: &Url) -> Result<PathBuf, Self::Error> {
        match self {
            None => get_path(url),
            Some(path) => Ok(path),
        }
    }
}
