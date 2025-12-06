use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Response;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;

const DOWNLOAD_DIR: &str = ".download";

/// Download a file from `url` into `./.download/<output>`.
pub fn download_file(url: &str, output: &str) -> Result<String> {
    println!("⬇️  Downloading: {}", url);

    let response: Response = reqwest::blocking::get(url).context("Failed to download URL")?;
    let total = response.content_length().unwrap_or(0);

    // Ensure download directory exists
    let download_dir = Path::new(DOWNLOAD_DIR);
    fs::create_dir_all(download_dir).context("Failed to create download directory")?;

    let pb = if total > 0 {
        ProgressBar::new(total)
    } else {
        ProgressBar::new_spinner()
    };

    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .context("Invalid progress bar template")?,
    );

    let mut src = response;

    let dest_path = download_dir.join(output);
    let mut dest = File::create(&dest_path)
        .with_context(|| format!("Failed to create destination file: {}", dest_path.display()))?;

    let mut writer = pb.wrap_write(&mut dest);
    copy(&mut src, &mut writer).context("Failed while copying download stream to file")?;
    pb.finish_with_message("Download completed!");

    println!("Saved to {}", dest_path.display());
    Ok(dest_path.to_string_lossy().to_string())
}
