use anyhow::{Context, Result};
use std::fs::create_dir;
use std::path::{Path, PathBuf};

pub fn create_local_dir(path: &str) -> Result<PathBuf> {
    let dir_path = Path::new(path).to_owned();
    if !dir_path.exists() {
        create_dir(&dir_path).context(format!("Failed to create directory \"{path}\""))?;
    }

    Ok(dir_path)
}
