// src/main.rs
/*
 * Main executable for BlockchainNftRegistryStudioPro
 */

use clap::Parser;
use blockchainnftregistrystudiopro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNftRegistryStudioPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
