use anyhow::{bail, Result};

static HEADER_END: &str = "%%";

pub struct Gabc {
    header: String,
    score: String,
}

impl Gabc {
    pub fn from_string(content: &str) -> Result<Self> {
        let file_parts = content.split(HEADER_END).collect::<Vec<&str>>();
        if file_parts.len() != 2 {
            bail!("Cannot find header separator");
        }

        Ok(Self {
            header: file_parts[0].to_string(),
            score: file_parts[1].to_string(),
        })
    }

    pub fn split_verses(&self) -> Vec<Self> {
        // Split score by line
        let mut verses = self
            .score
            .split('\n')
            .map(|line| line.trim())
            .collect::<Vec<&str>>();

        // Remove all empty lines.
        verses.retain(|line| !line.is_empty());

        // TODO: Finding clef requires better implementation.
        // Blindly assume clef is the first 4 characters of the first verse...
        let clef = &verses[0][..4];

        let mut gabc_verses: Vec<Self> = Vec::new();
        // First verse with clef already present.
        let first_verse = Gabc {
            header: self.header.clone(),
            score: verses[0].to_owned(),
        };
        gabc_verses.push(first_verse);

        // Include clef to the beginning of remaining verses.
        for v in &verses[1..] {
            let gabc = Gabc {
                header: self.header.clone(),
                score: clef.to_string() + v,
            };
            gabc_verses.push(gabc);
        }

        gabc_verses
    }

    pub fn to_string(self) -> String {
        self.header + HEADER_END + "\n" + &self.score + "\n"
    }
}
