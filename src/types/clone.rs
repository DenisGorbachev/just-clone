use git2::Repository;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

use crate::get_url;
use crate::Outcome;

pub async fn clone(url: Url, get_dir: impl FnOnce(&Url) -> Outcome<PathBuf>, stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome<Repository> {
    let repo_url = get_url(url).await?;
    let path_buf = get_dir(&repo_url)?;
    writeln!(stdout, "Cloning into {}", path_buf.display())?;
    let repo = Repository::clone(repo_url.as_str(), path_buf.as_path())?;
    Ok(repo)
}

pub async fn clone_from_str(url: &str, get_dir: impl FnOnce(&Url) -> Outcome<PathBuf>, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome<Repository> {
    clone(url.parse()?, get_dir, stdout, stderr).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::cli::convert_parent_dir_to_closure;
    use std::io::{stderr, stdout};
    use tempfile::tempdir;

    #[tokio::test]
    async fn must_clone() -> Outcome {
        let stdout = &mut stdout();
        let stderr = &mut stderr();
        // let impl_get_dir = GetDirFn::new(get_path);
        let tempdir = tempdir()?;
        let get_dir = convert_parent_dir_to_closure(Some(tempdir.path().to_owned()));
        let owner = "DenisGorbachev";
        let name = "url-macro";
        let url = format!("https://github.com/{owner}/{name}").parse()?;
        clone(url, get_dir, stdout, stderr).await?;
        let expected_repo_path = tempdir.path().join(owner).join(name);
        assert!(expected_repo_path.exists());
        Ok(())
    }
}
