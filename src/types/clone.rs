use crate::{GetPathError, GetUrlError, get_url};
use errgonomic::{handle, map_err};
use git2::Repository;
use std::io::{self, Write};
use std::path::PathBuf;
use thiserror::Error;
use url::Url;

pub async fn clone(url: Url, get_dir: impl FnOnce(&Url) -> Result<PathBuf, GetPathError>, _stdout: &mut impl Write, stderr: &mut impl Write) -> Result<Repository, CloneError> {
    use CloneError::*;
    let repo_url = handle!(get_url(url).await, GetUrlFailed);
    let path_buf = handle!(get_dir(&repo_url), GetDirFailed);
    handle!(writeln!(stderr, "Cloning into {}", path_buf.display()), WriteFailed);
    map_err!(Repository::clone(repo_url.as_str(), &path_buf), RepositoryCloneFailed, repo_url, path_buf)
}

#[derive(Error, Debug)]
pub enum CloneError {
    #[error("failed to get repo URL")]
    GetUrlFailed { source: GetUrlError },
    #[error("failed to get dir")]
    GetDirFailed { source: GetPathError },
    #[error("failed to write to stderr")]
    WriteFailed { source: io::Error },
    #[error("failed to clone '{repo_url}' into '{path_buf}'", path_buf = path_buf.display())]
    RepositoryCloneFailed { source: Box<git2::Error>, repo_url: Url, path_buf: PathBuf },
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::convert_parent_dir_to_closure;
    use std::io::{stderr, stdout};
    use tempfile::tempdir;
    use url_macro::url;

    #[tokio::test]
    async fn must_clone() -> Result<(), MustCloneError> {
        use MustCloneError::*;
        let stdout = &mut stdout();
        let stderr = &mut stderr();
        let tempdir = handle!(tempdir(), TempdirFailed);
        let get_dir = convert_parent_dir_to_closure(Some(tempdir.path().to_owned()));
        handle!(clone(url!("https://github.com/DenisGorbachev/url-macro"), get_dir, stdout, stderr).await, CloneFailed);
        assert!(tempdir.path().join("DenisGorbachev•url-macro").exists());
        Ok(())
    }

    #[derive(Error, Debug)]
    pub enum MustCloneError {
        #[error("failed to create a temp dir")]
        TempdirFailed { source: io::Error },
        #[error("failed to clone")]
        CloneFailed { source: CloneError },
    }
}
