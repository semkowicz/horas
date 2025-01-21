use crate::config;
use crate::divinum_officium::DivinumOfficium;
use crate::psalm_tone_tool::PsalmToneTool;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) struct PsalmBuilder {
    divinum_officium: DivinumOfficium,
    psalm_tone_tool: PsalmToneTool,
    target_dir: PathBuf,
    templates_dir: PathBuf,
}

impl PsalmBuilder {
    pub fn new() -> Result<Self> {
        let divinum_officium =
            DivinumOfficium::new().context("Failed to create DivinumOfficium")?;

        let psalm_tone_tool = PsalmToneTool::new().context("Failed to create PsalmToneTool")?;

        let templates_dir = config::data_dir()
            .context("Failed to construct PsalmToneTool directory path")?
            .join("templates");

        let target_dir = Path::new("./psalms").to_owned();

        Ok(Self {
            divinum_officium,
            psalm_tone_tool,
            target_dir,
            templates_dir,
        })
    }

    pub fn build_psalm(&self, number: u32, tone: &str) -> Result<()> {
        let psalm_tone = self.psalm_tone_tool.psalm(number, tone);
        let psalm_translation = self.divinum_officium.psalm(number)?;

        let filename_head = format!("{}-{}", number, tone);

        let first_verse = psalm_tone.first_verse_tone()?;
        let first_verse_path = self.target_dir.join(filename_head.clone() + ".gabc");
        fs::write(&first_verse_path, first_verse).context("Unable to write gabc file")?;

        let first_verse_tr = &psalm_translation[0];

        let verses_text = psalm_tone.verses_text()?;
        let verses_table = create_verses_table(&verses_text, &psalm_translation);

        let template: String = fs::read_to_string(self.templates_dir.join("psalm.tex.in"))
            .context("Failed to read tex template")?;

        let tex = template
            .replace(
                "@FIRST_VERSE_PATH@",
                &first_verse_path.to_string_lossy().to_string(),
            )
            .replace("@FIRST_VERSE_TRANSLATION@", first_verse_tr)
            .replace("@VERSES_TABLE@", &verses_table);

        let tex_path = self.target_dir.join(filename_head + ".tex");
        fs::write(tex_path, tex).context("Unable to write tex file")?;

        Ok(())
    }

    pub fn build_magnificat(&self, tone: &str) -> Result<()> {
        println!("Building Magnificat tone {}", tone);

        let magnificat_gabc = self
            .psalm_tone_tool
            .magnificat(tone)
            .gabc()
            .context("Failed to read Magnificat gabc")?;

        let target_dir = Path::new("./magnificat").to_owned();
        let target_path = target_dir.join(format!("Magnificat-{}.gabc", tone));
        fs::write(&target_path, magnificat_gabc)
            .context(format!("Unable to write {}", target_path.display()))?;

        Ok(())
    }
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
