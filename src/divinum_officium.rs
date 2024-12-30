use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub(crate) struct DivinumOfficium {
    psalter_dir: PathBuf,
}

impl DivinumOfficium {
    pub fn new() -> Self {
        let path = Path::new("/home/daniel/.local/share/horas/divinum-officium")
            .join("web/www/horas/Polski-Newer/Psalterium/Psalmorum");

        Self {
            psalter_dir: path.to_path_buf(),
        }
    }

    pub fn psalm(self, number: u32) -> Result<Vec<String>, String> {
        let filename = String::from("Psalm") + &number.to_string() + ".txt";
        let psalm_file = self.psalter_dir.join(filename);

        let file = File::open(psalm_file).map_err(|e| format!("Failed to open psalm file: {e}"))?;
        let buf = BufReader::new(file);

        let mut psalm_lines: Vec<String> = Vec::new();

        for line in buf.lines() {
            let line = match line {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to unwrap line: {e}")),
            };

            let text = remove_line_prefix(&line)?;

            psalm_lines.push(text.to_owned());
        }

        psalm_lines.push("Chwała Ojcu i Synowi, * i Duchowi Świętemu.".to_string());
        psalm_lines
            .push("Jak była na początku, teraz i zawsze, * i na wieki wieków. Amen.".to_string());

        Ok(psalm_lines)
    }
}

fn remove_line_prefix(line: &str) -> Result<&str, String> {
    let start = match line.find(' ') {
        None => return Err("Space character not found".to_string()),
        Some(v) => v,
    };
    let text = &line[start + 1..];

    Ok(text)
}
