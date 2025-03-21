use crate::dockerfile::DockerfileGenerator;
use anyhow::{Context, Result};
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecOptions};
use bollard::image::BuildImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use futures::TryStreamExt;
use log::{debug, info};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tempfile::TempDir;
use tokio::fs;
use tokio::time::timeout;

use crate::config::TIMEOUT_SECS;

/// Options for container creation
#[derive(Debug, Clone)]
pub struct ContainerOptions {
    pub distro: String,
    pub version: String,
    pub architecture: String,
    pub libc_variant: String,
    pub zip_path: PathBuf,
}

/// Manager for Docker containers
pub struct ContainerManager {
    docker: Docker,
    temp_dir: TempDir,
}

impl ContainerManager {
    /// Create a new container manager
    pub async fn new() -> Result<Self> {
        // Try to connect to Docker or Finch
        let docker = if cfg!(target_os = "macos") {
            // On macOS, try different connection methods
            Docker::connect_with_local_defaults()
                .or_else(|_| {
                    // Try to connect to Finch socket - try different possible locations
                    use bollard::ClientVersion;
                    let api_version = ClientVersion { major_version: 1, minor_version: 41 };
                    
                    // Try standard locations for Finch socket
                    Docker::connect_with_unix("/run/finch/finch.sock", 120, &api_version)
                        .or_else(|_| Docker::connect_with_unix("/var/run/finch/finch.sock", 120, &api_version))
                        .or_else(|_| Docker::connect_with_unix("/var/run/docker.sock", 120, &api_version))
                        .or_else(|_| Docker::connect_with_unix("~/.finch/docker.sock", 120, &api_version))
                        // Try with environment variable
                        .or_else(|_| {
                            // Try with direct TCP connection to Finch
                            Docker::connect_with_http("tcp://127.0.0.1:2375", 120, &api_version)
                        })
                })
                .or_else(|e| {
                    // If we can't connect, provide a more helpful error message
                    Err(anyhow::anyhow!("Failed to connect to Docker or Finch daemon. Make sure either Docker Desktop or Finch is installed and running. Error: {}", e))
                })?
        } else {
            // On other platforms, just try Docker
            Docker::connect_with_local_defaults()
                .or_else(|e| {
                    Err(anyhow::anyhow!("Failed to connect to Docker daemon. Make sure Docker is installed and running. Error: {}", e))
                })?
        };

        let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

        Ok(Self { docker, temp_dir })
    }

    /// Build a container image for the specified options
    pub async fn build_image(&self, options: &ContainerOptions) -> Result<String> {
        let tag = DockerfileGenerator::generate_tag(
            &options.distro,
            &options.version,
            &options.architecture,
            &options.libc_variant,
        );

        info!("Building container image: {}", tag);

        // Generate Dockerfile
        let dockerfile_content = DockerfileGenerator::generate(
            &options.distro,
            &options.version,
            &options.architecture,
        )?;

        // Write Dockerfile to temporary directory
        let dockerfile_path = self.temp_dir.path().join("Dockerfile");
        fs::write(&dockerfile_path, dockerfile_content)
            .await
            .context("Failed to write Dockerfile")?;

        // Build image
        let build_options = BuildImageOptions {
            dockerfile: "Dockerfile",
            t: &tag,
            buildargs: {
                let mut args = HashMap::new();
                args.insert("LIBC_VARIANT", options.libc_variant.as_str());
                args
            },
            ..Default::default()
        };

        let build_context = self.temp_dir.path().to_string_lossy().to_string();
        
        // Use TryStreamExt to process the build stream
        let build_stream = self
            .docker
            .build_image(build_options, None, Some(build_context.into()));

        let mut stream = build_stream;
        while let Some(output) = stream.try_next().await.map_err(|e| anyhow::anyhow!("Failed to build image: {}", e))? {
            if let Some(stream) = output.stream {
                debug!("{}", stream.trim());
            }
        }

        Ok(tag)
    }

    /// Run a container with the specified image tag
    pub async fn run_container(&self, tag: &str, zip_path: &Path, options: &ContainerOptions) -> Result<String> {
        info!("Running container: {}", tag);

        // Create a container name with test details and timestamp
        // Just use unix timestamp for simplicity without adding dependencies
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let container_name = format!(
            "q-test-{}-{}-{}-{}-{}",
            options.distro,
            options.version,
            options.architecture,
            options.libc_variant,
            timestamp
        );
        
        info!("Container name: {}", container_name);
        
        let create_options = CreateContainerOptions {
            name: container_name.as_str(),
            ..Default::default()
        };

        let host_config = HostConfig {
            binds: Some(vec![format!(
                "{}:/amazon-q-developer-cli/bundle/zip:ro",
                zip_path.to_string_lossy()
            )]),
            ..Default::default()
        };

        let config = Config {
            image: Some(tag),
            host_config: Some(host_config),
            ..Default::default()
        };

        let container = self
            .docker
            .create_container(Some(create_options), config)
            .await
            .context("Failed to create container")?;

        // Start container
        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .context("Failed to start container")?;

        Ok(container.id)
    }

    /// Execute a command in the container
    pub async fn exec_command(&self, container_id: &str, cmd: &[&str]) -> Result<(i64, String)> {
        let exec = self
            .docker
            .create_exec(
                container_id,
                CreateExecOptions {
                    cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    ..Default::default()
                },
            )
            .await
            .context("Failed to create exec")?;

        let output = self.docker
            .start_exec(&exec.id, None::<StartExecOptions>)
            .await
            .context("Failed to start exec")?;

        let output = match output {
            bollard::exec::StartExecResults::Attached { output, .. } => {
                let mut result = String::new();
                let mut stream = output;
                while let Some(chunk) = timeout(
                    Duration::from_secs(TIMEOUT_SECS),
                    stream.try_next(),
                )
                .await??
                {
                    match chunk {
                        bollard::container::LogOutput::StdOut { message } => {
                            result.push_str(&String::from_utf8_lossy(&message));
                        }
                        bollard::container::LogOutput::StdErr { message } => {
                            result.push_str(&String::from_utf8_lossy(&message));
                        }
                        _ => {}
                    }
                }
                result
            }
            _ => return Err(anyhow::anyhow!("Unexpected exec result")),
        };

        let inspect = self
            .docker
            .inspect_exec(&exec.id)
            .await
            .context("Failed to inspect exec")?;

        let exit_code = inspect.exit_code.unwrap_or(0);

        Ok((exit_code, output))
    }

    /// Clean up a container
    pub async fn cleanup_container(&self, container_id: &str) -> Result<()> {
        info!("Cleaning up container: {}", container_id);

        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker
            .remove_container(container_id, Some(options))
            .await
            .context("Failed to remove container")?;

        Ok(())
    }
}
