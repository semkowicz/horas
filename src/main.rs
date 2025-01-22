use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod config;
mod divinum_officium;
mod gabc;
mod psalm_builder;
mod psalm_tone_tool;
mod vespers;

/// Tool for creating Liturgia Horarum booklets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Vesperae configuration file
    #[clap(required = true)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    vespers::Vespers::load_from_file(&args.file)
        .context("Failed to load vespers configuration")?
        .build()
        .context("Failed to build vespers")
}
