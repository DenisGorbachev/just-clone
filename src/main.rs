use clap::Parser;
use just_clone::types::cli::Cli;
use just_clone::types::outcome::Outcome;
use std::io::{stderr, stdout};

#[tokio::main]
async fn main() -> Outcome {
    Cli::parse().run(&mut stderr(), &mut stdout()).await
}
