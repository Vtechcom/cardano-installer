use anyhow::*;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// Cross-platform executable permission setter
pub fn make_executable(path: &str) -> Result<()> {
    let p = Path::new(path);

    if !p.exists() {
        return Err(anyhow!("File không tồn tại: {}", path));
    }

    // Windows: không cần chmod, bỏ qua
    #[cfg(target_family = "windows")]
    {
        println!("ℹ️  Windows không cần chmod, bỏ qua: {}", path);
        return Ok(());
    }

    // Linux / macOS: chmod 755
    #[cfg(unix)]
    {
        let metadata = std::fs::metadata(p)?;
        let mut perms = metadata.permissions();

        // Set executable bit
        perms.set_mode(0o755);

        std::fs::set_permissions(p, perms)?;

        println!("🔧 Đã chmod +x: {}", path);
        return Ok(());
    }

    Ok(())
}
