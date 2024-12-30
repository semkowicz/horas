use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub(crate) struct PsalmTone {
    tones_dir: PathBuf,
}

impl PsalmTone {
    pub fn new() -> Self {
        let path = Path::new("/home/daniel/.local/share/horas/psalms");

        Self {
            tones_dir: path.to_path_buf(),
        }
    }

    pub fn psalm(self, number: u32, tone: &str) -> Result<Vec<String>, String> {
        let filename = format!("{number}-{tone}.tex");
        let psalm_file = self.tones_dir.join(filename);

        let file =
            File::open(psalm_file).map_err(|e| format!("Failed to open psalm tone file: {e}"))?;
        let buf = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in buf.lines() {
            let line = match line {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to unwrap line: {e}")),
            };

            let trimmed = line.trim_start_matches("\\item ");

            lines.push(trimmed.to_owned());
        }

        Ok(lines)
    }
}
