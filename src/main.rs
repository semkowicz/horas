use clap::Parser;

mod divinum_officium;
mod psalm_tone;

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

fn main() -> Result<(), String> {
    let args = Args::parse();

    let translations = divinum_officium::DivinumOfficium::new();
    let tones = psalm_tone::PsalmTone::new();

    println!("Psalm tone:");
    for line in tones.psalm(args.psalm, &args.tone)? {
        println!("{line}");
    }

    println!();
    println!("Translation:");
    for line in translations.psalm(args.psalm)? {
        println!("{line}");
    }

    Ok(())
}
