use colored::*;

use anyhow::Result;

#[derive(Debug, serde::Deserialize)]
pub struct MithrilConfig {
    pub mithril: MithrilSection,
}

#[derive(Debug, serde::Deserialize)]
pub struct MithrilSection {
    pub version: String,
    pub base_url: String,
    pub binaries: std::collections::HashMap<String, String>,
}

const MITHRIL_CONFIG: &str = include_str!("mithril.yaml");

 fn load_mithril_config() -> Result<MithrilConfig> {
    let cfg: MithrilConfig = serde_yaml::from_str(MITHRIL_CONFIG)?;
    Ok(cfg)
}


pub fn detect_mithril_key() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match (os, arch) {
        ("windows", "x86_64") => "windows-x64".to_string(),
        ("linux", "x86_64") => "linux-x64".to_string(),
        ("linux", "aarch64") => "linux-arm64".to_string(),
        ("macos", "x86_64") => "macos-x64".to_string(),
        ("macos", "aarch64") => "macos-arm64".to_string(),
        _ => format!("{os}-{arch}"),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct MithrilBinaryInfo {
    pub url: String,
    pub version: String,
    pub key: String,
    pub filename: String,
}

pub fn load_mithril_config_by_os() -> Result<MithrilBinaryInfo> {
    let config = load_mithril_config()?;
    let key = detect_mithril_key();
    let bin = config
        .mithril
        .binaries
        .get(&key)
        .ok_or_else(|| anyhow::anyhow!("Không tìm thấy binary URL cho key: {}", key))?;
    let base = config.mithril.base_url.trim_end_matches('/');
    let url = format!("{}/{}", base, bin);
    println!("Mithril version: {}", config.mithril.version);
    println!("{} {}", "Mithril client:", url.cyan());
    Ok(MithrilBinaryInfo {
        url,
        version: config.mithril.version,
        key: key.to_string(),
        filename: bin.to_string(),
    })
}
