use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use derive_more::Error;
use fmt_derive::Display;
use git2::Repository;
use url::Url;

use crate::crates_io::get_repo_url_from_crates_io_url;
use crate::types::command::Command;
use crate::types::outcome::Outcome;

#[derive(Parser, Clone, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(value_parser = clap::value_parser!(Url))]
    url: Url,

    dir: Option<PathBuf>,
}

impl Cli {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self.command {
            None => just_clone(self.url, self.dir, stdout, stderr).map(|_| ()),
            Some(command) => command.run(stdout, stderr).await,
        }
    }
}

pub fn just_clone(url: Url, dir: Option<PathBuf>, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome<Repository> {
    let repo_url = get_url(url)?;
    let path = match dir {
        None => get_path(&repo_url)?,
        Some(path) => path,
    };
    let repo = Repository::clone(repo_url.as_str(), path)?;
    Ok(repo)
}

pub fn get_url(url: Url) -> Outcome<Url> {
    match url.domain() {
        Some("crates.io") => get_repo_url_from_crates_io_url(&url).map_err(From::from),
        Some("docs.rs") => todo!(),
        Some("github.com") => todo!(),
        Some(_) => todo!(),
        None => todo!(),
    }
}

pub fn get_path(url: &Url) -> Outcome<PathBuf> {
    match url.domain() {
        Some("github.com") => get_path_from_github_url(url),
        Some(_) => todo!(),
        None => todo!(),
    }
}

pub fn get_path_from_github_url(url: &Url) -> Outcome<PathBuf> {
    let mut segments = url.path_segments().ok_or(PathNotFound)?;
    let owner = segments.next().ok_or(OwnerNotFound)?;
    let name = segments.next().ok_or(NameNotFound)?;
    let path_buf = [owner, name].iter().collect();
    Ok(path_buf)
}

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct PathNotFound;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct OwnerNotFound;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NameNotFound;

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
