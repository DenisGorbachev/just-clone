use clap::Parser;
use just_clone::Cli;
use just_clone::Outcome;
use std::io::{stderr, stdout};

#[tokio::main]
async fn main() -> Outcome {
    Cli::parse().run(&mut stderr(), &mut stdout()).await
}
