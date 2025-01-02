use clap::Parser;
use std::fs;
use std::path::Path;

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

    let filename = format!("{}-{}", args.psalm, args.tone);
    let target_dir = Path::new("./psalms");

    psalm_tone::PsalmTone::new().install_gabc(args.psalm, &args.tone, target_dir)?;

    let tex = generate_tex(args.psalm, &args.tone)?;
    let tex_path = target_dir.join(filename + ".tex");
    fs::write(tex_path, tex).map_err(|e| format!("Unable to write file: {e}"))?;

    Ok(())
}

fn create_verses_table(tone: &Vec<String>, translation: &Vec<String>) -> String {
    let mut table = String::new();

    for i in 0..tone.len() {
        table += &format!("{} &\n", i + 2);
        table += &format!("{} &\n", tone[i]);
        table += &format!("{} \\\\\n", translation[i + 1]);
    }

    table
}

fn generate_tex(psalm: u32, tone: &str) -> Result<String, String> {
    let translations = divinum_officium::DivinumOfficium::new();
    let tones = psalm_tone::PsalmTone::new();

    let psalm_tone = tones.psalm(psalm, tone)?;
    let psalm_translation = translations.psalm(psalm)?;

    let first_verse_path = format!("psalms/{}-{}.gabc", psalm, tone);
    let first_verse_tr = &psalm_translation[0];
    let verses_table = create_verses_table(&psalm_tone, &psalm_translation);

    let template_path = Path::new("/home/daniel/.local/share/horas/templates/psalm.tex.in");
    let template: String = fs::read_to_string(template_path)
        .map_err(|e| format!("Failed to read tex template: {e}"))?;

    let tex = template
        .replace("@FIRST_VERSE_PATH@", &first_verse_path)
        .replace("@FIRST_VERSE_TRANSLATION@", first_verse_tr)
        .replace("@VERSES_TABLE@", &verses_table);

    Ok(tex)
}
