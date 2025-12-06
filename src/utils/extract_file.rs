use flate2::read::GzDecoder;
use tar::Archive;
use anyhow::Result;
use std::fs::File;


pub fn extract_tar_gz(archive_path: &str, output_dir: &str) -> Result<()> {
    println!("📦 Extracting {} → {}", archive_path, output_dir);

    let tar_gz = File::open(archive_path)?;
    let decompressor = GzDecoder::new(tar_gz);

    let mut archive = Archive::new(decompressor);
    archive.unpack(output_dir)?;

    println!("✅ Extracted to {}", output_dir);
    Ok(())
}
