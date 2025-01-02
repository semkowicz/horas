use crate::psalm_builder::PsalmBuilder;
use anyhow::{Context, Result};
use clap::Parser;

mod config;
mod divinum_officium;
mod psalm_builder;
mod psalm_tone_tool;

/// Tool for creating Liturgia Horarum booklets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Psalm number
    #[clap(required = true)]
    psalm: u32,

    /// Tone
    #[clap(required = true)]
    tone: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let psalm_builder = PsalmBuilder::new().context("Failed to create PsalmBuilder")?;

    psalm_builder
        .build_psalm(args.psalm, &args.tone)
        .context("Failed to build psalm")
}
