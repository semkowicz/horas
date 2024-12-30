use crate::config;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub(crate) struct DivinumOfficium {
    psalter_dir: PathBuf,
}

impl DivinumOfficium {
    pub fn new() -> Result<Self> {
        let psalter_dir = config::data_dir()
            .context("Failed to construct DivinumOfficium directory path")?
            .join("divinum-officium")
            .join("web/www/horas/Polski-Newer/Psalterium/Psalmorum");

        Ok(Self { psalter_dir })
    }

    pub fn psalm(&self, number: u32) -> Result<Vec<String>> {
        let filename = String::from("Psalm") + &number.to_string() + ".txt";
        let psalm_file = self.psalter_dir.join(filename);

        let file = File::open(psalm_file).context("Failed to open psalm file")?;
        let buf = BufReader::new(file);

        let mut psalm_lines: Vec<String> = Vec::new();

        for line in buf.lines() {
            let line = line.context("Failed to unwrap line")?;
            let text = remove_line_prefix(&line)?;
            psalm_lines.push(text.to_owned());
        }

        psalm_lines.push("Chwała Ojcu i Synowi, * i Duchowi Świętemu.".to_string());
        psalm_lines
            .push("Jak była na początku, teraz i zawsze, * i na wieki wieków. Amen.".to_string());

        Ok(psalm_lines)
    }
}

fn remove_line_prefix(line: &str) -> Result<&str> {
    let start = line.find(' ').context("Space character not found")?;
    let text = &line[start + 1..];

    Ok(text)
}
