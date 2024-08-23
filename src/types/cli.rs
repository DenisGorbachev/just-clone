use std::io::Write;
use std::path::PathBuf;

use crate::get_path;
use crate::types::clone::clone;
use crate::types::command::Command;
use crate::Outcome;
use clap::Parser;
use derive_more::Error;
use fmt_derive::Display;
use url::Url;

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
            None => clone(self.url, convert_target_dir_to_closure(self.dir), stdout, stderr)
                .await
                .map(|_| ()),
            Some(command) => command.run(stdout, stderr).await,
        }
    }
}

pub fn convert_target_dir_to_closure(target_dir: Option<PathBuf>) -> impl FnOnce(&Url) -> Outcome<PathBuf> {
    |url| match target_dir {
        None => get_path(url),
        Some(path) => Ok(path),
    }
}

pub fn convert_parent_dir_to_closure(parent_dir: Option<PathBuf>) -> impl FnOnce(&Url) -> Outcome<PathBuf> {
    |url| match parent_dir {
        None => get_path(url),
        Some(mut path) => {
            path.push(get_path(url)?);
            Ok(path)
        }
    }
}

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct PathNotFoundError;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct OwnerNotFoundError;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NameNotFoundError;

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
