use crate::{CloneError, Command, GetPathError, clone, get_path};
use clap::Parser;
use errgonomic::handle;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;
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
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Result<ExitCode, CliRunError> {
        use CliRunError::*;
        match self.command {
            None => {
                handle!(clone(self.url, convert_target_dir_to_closure(self.dir), stdout, stderr).await, CloneFailed);
                Ok(ExitCode::SUCCESS)
            }
            Some(command) => Ok(command.run(stdout, stderr).await),
        }
    }
}

#[derive(Error, Debug)]
pub enum CliRunError {
    #[error("failed to clone")]
    CloneFailed { source: CloneError },
}

pub fn convert_target_dir_to_closure(target_dir: Option<PathBuf>) -> impl FnOnce(&Url) -> Result<PathBuf, GetPathError> {
    |url| match target_dir {
        None => get_path(url),
        Some(path) => Ok(path),
    }
}

pub fn convert_parent_dir_to_closure(parent_dir: Option<PathBuf>) -> impl FnOnce(&Url) -> Result<PathBuf, GetPathError> {
    |url| match parent_dir {
        None => get_path(url),
        Some(mut path) => {
            path.push(get_path(url)?);
            Ok(path)
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
