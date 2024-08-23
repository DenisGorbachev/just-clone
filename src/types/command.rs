use std::io::Write;

use crate::Outcome;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub enum Command {}

impl Command {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        match self {}
    }
}
