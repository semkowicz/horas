use clap::Parser;

mod divinum_officium;

/// Tool for creating Liturgia Horarum booklets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Psalm number
    #[clap(required = true)]
    psalm: u32,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let translations = divinum_officium::DivinumOfficium::new();

    for line in translations.psalm(args.psalm)? {
        println!("{line}");
    }

    Ok(())
}
