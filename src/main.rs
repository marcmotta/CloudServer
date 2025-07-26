// src/main.rs
/*
 * Main executable for CloudServer
 */

use clap::Parser;
use cloudserver::{Result, run};

#[derive(Parser)]
#[command(version, about = "CloudServer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
