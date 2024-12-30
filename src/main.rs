use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("Psalm: {}, tone: {}", args.psalm, args.tone);
}
