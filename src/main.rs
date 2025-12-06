use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::{fs, path::PathBuf};
use which::which;

mod configs;
mod utils;
use configs::mithril;
use utils::download_file::download_file;
use utils::extract_file::extract_tar_gz;

#[derive(Parser, Debug)]
#[command(name = "cardano-installer")]
#[command(about = "✨ Cardano node setup CLI (Docker + Mithril)", long_about = None)]
struct Cli {
    /// Chế độ verbose
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    show_banner();

    ensure_docker(&cli)?;

    if let Some(info) = find_cardano_node_container()? {
        println!(
            "{} Đã tìm thấy {} với trạng thái: {}",
            "✓".green(),
            info.name.cyan().bold(),
            info.status.yellow()
        );
        println!("   Image: {}", info.image);
        println!("   Ports: {}", info.ports);
    } else {
        println!("{}", "⚠ Không tìm thấy container cardano-node".yellow());
        let install = Confirm::new()
            .with_prompt("Bạn có muốn tiến hành cài đặt cardano-node bằng Docker không?")
            .default(true)
            .interact()?;

        if !install {
            println!("{}", "Thoát, không cài đặt cardano-node.".red());
            return Ok(());
        }

        run_cardano_setup_flow(&cli)?;
    }

    Ok(())
}

fn show_banner() {
    println!(
        "\n{}\n{}\n",
        "==============================".bright_black(),
        "  Hydra / Cardano Node Setup  ".bold().bright_blue()
    );
    println!(
        "{} {}",
        "➤".bright_green(),
        "Tool hỗ trợ cài đặt cardano-node (Docker + Mithril Snapshot)".bright_white()
    );
    println!("{}", "==============================\n".bright_black());
}

fn ensure_docker(cli: &Cli) -> Result<()> {
    print!("{}", "⏳ Kiểm tra Docker... ".bright_white());
    if which("docker").is_err() {
        println!("{}", "KHÔNG TÌM THẤY".red().bold());

        let install = Confirm::new()
            .with_prompt("Docker chưa được cài. Bạn có muốn cài Docker tự động (Linux) không?")
            .default(true)
            .interact()?;

        if !install {
            anyhow::bail!("Không thể tiếp tục nếu không có Docker");
        }

        install_docker(cli)?;
    } else {
        println!("{}", "OK".green().bold());
        show_docker_version(cli)?;
    }

    Ok(())
}

fn install_docker(cli: &Cli) -> Result<()> {
    println!("{}", "🚀 Bắt đầu cài Docker (Linux)…".bright_blue());
    println!(
        "{}",
        "Lưu ý: Script này tương đương `curl -fsSL https://get.docker.com | sh`"
            .bright_black()
    );

    let confirm = Confirm::new()
        .with_prompt("Bạn chắc chắn muốn chạy script cài Docker này chứ?")
        .default(true)
        .interact()?;

    if !confirm {
        anyhow::bail!("User huỷ cài Docker");
    }

    // Chỉ demo: thực tế nên handle OS cụ thể, require sudo...
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://get.docker.com | sh")
        .status()
        .context("Không thể chạy script cài Docker")?;

    if !status.success() {
        anyhow::bail!("Cài Docker thất bại, code: {:?}", status.code());
    }

    println!("{}", "✅ Docker đã được cài đặt xong.".green().bold());
    if cli.verbose {
        show_docker_version(cli)?;
    }

    Ok(())
}

fn show_docker_version(_cli: &Cli) -> Result<()> {
    let output = Command::new("docker")
        .arg("--version")
        .output()
        .context("Không thể chạy docker --version")?;

    if output.status.success() {
        let ver = String::from_utf8_lossy(&output.stdout);
        println!("   Docker version: {}", ver.trim().bright_cyan());
    } else {
        println!("{}", "Không thể lấy version Docker".yellow());
    }

    Ok(())
}

struct ContainerInfo {
    name: String,
    image: String,
    status: String,
    ports: String,
}

fn find_cardano_node_container() -> Result<Option<ContainerInfo>> {
    let output = Command::new("docker")
        .args(&["ps", "-a", "--format", "{{.Names}}||{{.Image}}||{{.Status}}||{{.Ports}}"])
        .output()
        .context("Không thể chạy docker ps")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split("||").collect();
        if parts.len() < 4 {
            continue;
        }
        let name = parts[0].to_string();
        let image = parts[1].to_string();
        let status = parts[2].to_string();
        let ports = parts[3].to_string();

        if image.contains("cardano-node") || name.contains("cardano-node") {
            return Ok(Some(ContainerInfo {
                name,
                image,
                status,
                ports,
            }));
        }
    }

    Ok(None)
}

fn run_cardano_setup_flow(_cli: &Cli) -> Result<()> {
    println!("{}", "\n🚀 Bắt đầu flow cài cardano-node…".bright_blue());

    let network = select_network()?;
    println!("   Network đã chọn: {}\n", network.cyan().bold());

    download_network_config_stub(&network)?;

    let compose_path = generate_docker_compose(&network)?;
    println!(
        "{} {}",
        "✓ Đã tạo docker-compose tại".green(),
        compose_path.to_string_lossy().bright_cyan()
    );

    ensure_mithril_client()?;

    run_mithril_snapshot(&network)?;

    docker_compose_up(&compose_path)?;

    println!("{}", "\n🎉 Hoàn tất setup cardano-node!".bright_green().bold());
    Ok(())
}

fn select_network() -> Result<String> {
    let options = vec!["mainnet", "preprod", "sanchonet", "custom"];
    let selection = Select::new()
        .with_prompt("Chọn Cardano network bạn muốn sử dụng")
        .items(&options)
        .default(1)
        .interact()?;

    if options[selection] == "custom" {
        // Có thể dùng Input để hỏi tên network cụ thể
        Ok("custom".to_string())
    } else {
        Ok(options[selection].to_string())
    }
}

fn download_network_config_stub(network: &str) -> Result<()> {
    println!(
        "{} {}",
        "⏳ (Stub) Tải cấu hình mạng cho".bright_white(),
        network.cyan()
    );
    println!(
        "{}",
        "→ TODO: Ania sẽ implement download config từ URL sau."
            .bright_black()
    );
    Ok(())
}

fn generate_docker_compose(network: &str) -> Result<PathBuf> {
    let compose_content = format!(
        r#"
version: "3.9"
services:
  cardano-node:
    image: inputoutput/cardano-node:latest
    container_name: cardano-node-{network}
    restart: always
    ports:
      - "3001:3001"
    volumes:
      - ./config/{network}:/config
      - ./db/{network}:/db
    command: >
      cardano-node run
        --config /config/config.json
        --topology /config/topology.json
        --database-path /db
        --socket-path /db/node.socket
"#,
        network = network
    );

    let path = PathBuf::from("docker-compose.yml");
    fs::write(&path, compose_content)?;
    Ok(path)
}

fn ensure_mithril_client() -> Result<()> {
    println!("{}", "\n⏳ Download Mithril client…".bright_white());
    let confirm = Confirm::new()
        .with_prompt("Bạn có muốn tải mithril-client cho OS hiện tại không?")
        .default(true)
        .interact()?;
    if !confirm {
        anyhow::bail!("Không có mithril-client → dừng setup snapshot DB");
    }

    download_mithril_client()?;
    Ok(())
}

fn download_mithril_client() -> Result<()> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    println!(
        "{} {} / {}",
        "→ Hệ điều hành hiện tại:".bright_white(),
        os.cyan(),
        arch.cyan()
    );

    let mithril_conf = mithril::load_mithril_config_by_os()?;
    println!("{}", mithril_conf.version.bright_green());
    // Download mithril client theo URL trên...
    let downloaded_path = download_file(&mithril_conf.url, &mithril_conf.filename)?;
    extract_tar_gz(
        &downloaded_path,
        "./mithril-client",
    )?;
    utils::make_executable::make_executable("./mithril-client/mithril-client.exe")?;
    println!("🎉 Mithril client is ready at: {}", "./mithril-client");

    Ok(())
}

fn run_mithril_snapshot(network: &str) -> Result<()> {
    println!(
        "{} {}",
        "⏳ Chạy mithril-client để sync snapshot DB cho network"
            .bright_white(),
        network.cyan()
    );

    // TODO: Ania bổ sung command thực:
    // mithril-client cardano-db download --network <network> --dest ./db/<network>
    println!(
        "{}",
        "TODO: Gọi mithril-client cardano-db download ... (stub)"
            .bright_black()
    );

    Ok(())
}

fn docker_compose_up(path: &PathBuf) -> Result<()> {
    println!(
        "{} {}",
        "🚀 Chạy docker compose up với file".bright_white(),
        path.to_string_lossy().cyan()
    );

    let status = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(path)
        .arg("up")
        .arg("-d")
        .status()
        .context("Không thể chạy docker compose up")?;

    if !status.success() {
        anyhow::bail!("docker compose up thất bại với code: {:?}", status.code());
    }

    println!("{}", "✅ cardano-node đã được khởi chạy!".green().bold());
    Ok(())
}
