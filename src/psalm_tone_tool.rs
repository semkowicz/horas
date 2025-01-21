use crate::config;
use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub(crate) struct PsalmToneTool {
    psalms_dir: PathBuf,
    canticles_gabc_dir: PathBuf,
}

impl PsalmToneTool {
    pub fn new() -> Result<Self> {
        let ptt_dir = config::data_dir()
            .context("Failed to construct PsalmToneTool directory path")?
            .join("psalm-tone-tool");

        let psalms_dir = ptt_dir.join("psalms");
        let canticles_gabc_dir = ptt_dir.join("canticles-gabc");

        Ok(Self {
            psalms_dir,
            canticles_gabc_dir,
        })
    }

    pub fn psalm(&self, number: u32, tone: &str) -> Piece {
        let filename_head = format!("{number}-{tone}");
        let gabc_path = self.psalms_dir.join(filename_head.clone() + ".gabc");
        let tex_path = self.psalms_dir.join(filename_head + ".tex");

        Piece {
            gabc_path,
            tex_path,
        }
    }

    pub fn magnificat(&self, tone: &str) -> Piece {
        let filename_head = format!("Magnificat-{tone}");
        let gabc_path = self
            .canticles_gabc_dir
            .join(filename_head.clone() + ".gabc");
        let tex_path = PathBuf::new();

        Piece {
            gabc_path,
            tex_path,
        }
    }
}

pub(crate) struct Piece {
    gabc_path: PathBuf,
    tex_path: PathBuf,
}

impl Piece {
    pub fn first_verse_tone(&self) -> Result<String> {
        fs::read_to_string(&self.gabc_path).context("Failed to read tone line file")
    }

    pub fn gabc(&self) -> Result<String> {
        fs::read_to_string(&self.gabc_path).context(format!(
            "Failed to read gabc file \"{}\"",
            &self.gabc_path.display()
        ))
    }

    pub fn verses_text(&self) -> Result<Vec<String>> {
        let file = File::open(&self.tex_path).context("Failed to open psalm tone file")?;
        let buf = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in buf.lines() {
            let line = line.context("Failed to unwrap line")?;
            let trimmed = line.trim_start_matches("\\item ");
            lines.push(trimmed.to_owned());
        }

        Ok(lines)
    }
}
