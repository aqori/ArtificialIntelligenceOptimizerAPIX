// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerAPIX
 */

use clap::Parser;
use artificialintelligenceoptimizerapix::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerAPIX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
