use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Find all ZIP files in the specified directory
pub async fn find_zip_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    
    let mut entries = fs::read_dir(dir)
        .await
        .context("Failed to read directory")?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "zip") {
            result.push(path);
        }
    }
    
    if result.is_empty() {
        return Err(anyhow::anyhow!("No ZIP files found in {}", dir.display()));
    }
    
    Ok(result)
}

/// Parse architecture and libc variant from a ZIP filename
/// Expected format: amazon-q-developer-cli-{arch}-linux[-musl].zip
pub fn parse_zip_filename(path: &Path) -> Option<(String, String)> {
    let filename = path.file_name()?.to_str()?;
    
    // Extract architecture and libc variant using string operations
    if !filename.starts_with("amazon-q-developer-cli-") || !filename.ends_with(".zip") {
        return None;
    }
    
    // Remove prefix and suffix
    let middle = filename
        .strip_prefix("amazon-q-developer-cli-")?
        .strip_suffix(".zip")?;
    
    // Check for musl variant
    if middle.ends_with("-linux-musl") {
        let arch = middle.strip_suffix("-linux-musl")?;
        return Some((arch.to_string(), "musl".to_string()));
    } else if middle.ends_with("-linux") {
        let arch = middle.strip_suffix("-linux")?;
        return Some((arch.to_string(), "glibc".to_string()));
    }
    
    None
}

/// Check if a command is available in the system
pub fn is_command_available(command: &str) -> bool {
    which::which(command).is_ok()
}

/// Check which container runtime is available
pub fn check_container_runtime() -> Result<String> {
    // Debug: Print environment variables related to Docker
    if let Ok(docker_host) = std::env::var("DOCKER_HOST") {
        println!("DOCKER_HOST environment variable is set to: {}", docker_host);
    } else {
        println!("DOCKER_HOST environment variable is not set");
    }
    
    // For Finch, we need to check if the VM is running
    if is_command_available("finch") {
        println!("Finch is installed");
        return Ok("finch".to_string());
    }
    
    if is_command_available("docker") {
        println!("Docker is installed");
        return Ok("docker".to_string());
    } else if is_command_available("podman") {
        println!("Podman is installed");
        return Ok("podman".to_string());
    } else if cfg!(target_os = "macos") && std::path::Path::new("/run/finch/finch.sock").exists() {
        println!("Finch socket found at /run/finch/finch.sock");
        return Ok("finch".to_string());
    } else if cfg!(target_os = "macos") && std::path::Path::new("/var/run/finch/finch.sock").exists() {
        println!("Finch socket found at /var/run/finch/finch.sock");
        return Ok("finch".to_string());
    } else if cfg!(target_os = "macos") && std::path::Path::new("/var/run/docker.sock").exists() {
        println!("Docker socket found at /var/run/docker.sock");
        return Ok("docker".to_string());
    } else {
        Err(anyhow::anyhow!("No container runtime found. Please install Docker, Podman, or Finch."))
    }
}
