use anyhow::{Context, Result};
use clap::Parser;
use divinum_officium::DivinumOfficium;

mod config;
mod divinum_officium;

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

    println!("Psalm: {}, tone: {}", args.psalm, args.tone);
    println!();

    let divinum_officium = DivinumOfficium::new().context("Failed to create DivinumOfficium")?;

    for line in divinum_officium.psalm(1)? {
        println!("{line}");
    }

    Ok(())
}
