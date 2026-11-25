use anyhow::{Context, Result, bail};
use std::fs::{File, create_dir};
use std::io;
use std::path::{Path, PathBuf};
use url::Url;

pub fn fetch(url: &Url) -> Result<PathBuf> {
    let downloads_dir = Path::new("downloads");
    if !downloads_dir.exists() {
        create_dir(downloads_dir).context("Failed to create downloads directory")?
    }

    let (resolved_url, file_name) = match url.scheme() {
        "gregobase" => gregobase_to_full_url(url)?,
        _ => {
            bail!("Unsupported URL scheme")
        }
    };

    let file_path = downloads_dir.join(file_name);
    if !file_path.exists() {
        fetch_http(&resolved_url, &file_path)?;
    }

    Ok(file_path)
}

fn gregobase_to_full_url(url: &Url) -> Result<(Url, String)> {
    let id = url.host_str().context("ID not found in gregobase URI")?;

    let url = Url::parse_with_params(
        "https://gregobase.selapa.net/download.php?format=gabc",
        &[("id", id)],
    )?;

    let file_name = format!("gregobase-{id}.gabc");

    Ok((url, file_name))
}

fn fetch_http(url: &Url, file_path: &Path) -> Result<()> {
    println!("Fetching \"{url}\"");
    let mut response = ureq::get(url.as_str())
        .call()
        .context("Failed to download file")?;
    let mut reader = response.body_mut().as_reader();
    let mut file = File::create(file_path).context("Failed to create file")?;

    io::copy(&mut reader, &mut file)?;
    Ok(())
}
