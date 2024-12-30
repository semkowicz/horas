use anyhow::{Error, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn data_dir() -> Result<PathBuf> {
    let data_dir = ProjectDirs::from("pl", "Semkowicz", "Horas")
        .ok_or_else(|| Error::msg("No valid home directory path"))?
        .data_dir()
        .to_owned();

    Ok(data_dir)
}
