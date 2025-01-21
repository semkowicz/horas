use crate::psalm_builder::PsalmBuilder;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Psalm {
    number: u32,
    tone: String,
}

#[derive(Deserialize)]
struct Magnificat {
    tone: String,
}

#[derive(Deserialize)]
pub struct Vespers {
    psalms: Vec<Psalm>,
    magnificat: Magnificat,
}

impl Vespers {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let vespers_str =
            fs::read_to_string(path).context(format!("Failed to read file {}", path.display()))?;

        let vespers: Vespers =
            toml::from_str(&vespers_str).context("Failed to parse vesperae configuration")?;

        Ok(vespers)
    }

    pub fn build(self) -> Result<()> {
        let psalm_builder = PsalmBuilder::new().context("Failed to create PsalmBuilder")?;

        for psalm in self.psalms {
            println!("Building psalm {} tone {}", psalm.number, psalm.tone);

            psalm_builder
                .build_psalm(psalm.number, &psalm.tone)
                .context(format!("Failed to build psalm {}", psalm.number))?;
        }

        psalm_builder
            .build_magnificat(&self.magnificat.tone)
            .context("Failed to build magnificat")?;

        Ok(())
    }
}
