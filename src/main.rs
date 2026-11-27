use clap::Parser;
use errgonomic::exit_result;
use just_clone::Cli;
use std::io::{stderr, stdout};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let result = Cli::parse().run(&mut stdout(), &mut stderr()).await;
    exit_result(result)
}
