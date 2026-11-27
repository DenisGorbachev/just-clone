use std::io::Write;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Autocomplete,
}

use Command::*;

impl Command {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> ExitCode {
        match self {
            Autocomplete => todo!(),
        }
    }
}
