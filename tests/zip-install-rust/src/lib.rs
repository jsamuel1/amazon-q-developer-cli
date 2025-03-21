use std::collections::HashMap;
use std::path::Path;

use anyhow::{
    Result,
    anyhow,
};
use bollard::Docker;
use bollard::container::{
    Config,
    CreateContainerOptions,
    RemoveContainerOptions,
    StartContainerOptions,
};
use bollard::image::{
    BuildImageOptions,
    // CreateImageOptions, // Removing unused import
};
use futures_util::stream::StreamExt;
use log::{
    // debug, // Removing unused import
    error,
    info,
    warn,
};
// use serde::Deserialize; // Removing unused import
use tempfile::TempDir;
use uuid::Uuid;

// Re-export for tests
pub use crate::dockerfile::DockerfileGenerator;

mod dockerfile;

/// Distribution configuration
#[derive(Debug, Clone)]
pub struct DistributionConfig {
    pub name: String,
    pub version: String,
    pub architectures: Vec<String>,
    pub libc_variants: Vec<String>,
}

/// Get all supported distributions for testing
pub fn get_distributions() -> Vec<DistributionConfig> {
    vec![
        DistributionConfig {
            name: "ubuntu".to_string(),
            version: "22.04".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "ubuntu".to_string(),
            version: "20.04".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "debian".to_string(),
            version: "12".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "debian".to_string(),
            version: "11".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "amazonlinux".to_string(),
            version: "2023".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "amazonlinux".to_string(),
            version: "2".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "alpine".to_string(),
            version: "3.19".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["musl".to_string()],
        },
        DistributionConfig {
            name: "alpine".to_string(),
            version: "3.18".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["musl".to_string()],
        },
        DistributionConfig {
            name: "fedora".to_string(),
            version: "39".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "fedora".to_string(),
            version: "38".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "rockylinux".to_string(),
            version: "9".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        DistributionConfig {
            name: "rockylinux".to_string(),
            version: "8".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
    ]
}

/// Test runner for installation tests
pub struct TestRunner {
    docker: Docker,
    temp_dir: TempDir,
    keep_containers: bool,
}

impl TestRunner {
    /// Create a new test runner
    pub async fn new(_zip_dir: impl AsRef<Path>) -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        let temp_dir = tempfile::tempdir()?;

        Ok(Self {
            docker,
            temp_dir,
            keep_containers: false,
        })
    }

    /// Set whether to keep containers after tests
    pub fn with_keep_containers(mut self, keep: bool) -> Self {
        self.keep_containers = keep;
        self
    }

    /// Run a test for a specific distribution
    pub async fn run_test(
        &self,
        distro: &str,
        version: &str,
        arch: &str,
        libc: &str,
        zip_dir: impl AsRef<Path>,
    ) -> Result<bool> {
        // Find the appropriate ZIP file
        let zip_file = if libc == "musl" {
            format!("amazon-q-developer-cli-{}-linux-musl.zip", arch)
        } else {
            format!("amazon-q-developer-cli-{}-linux.zip", arch)
        };

        let zip_path = zip_dir.as_ref().join(&zip_file);
        if !zip_path.exists() {
            return Err(anyhow!("ZIP file not found: {}", zip_path.display()));
        }

        // Generate a unique container name
        let container_name = format!("q-test-{}", Uuid::new_v4());

        // Generate Dockerfile
        let dockerfile = DockerfileGenerator::generate(distro, version, arch)?;

        // Create temporary directory for Docker build context
        let context_dir = self.temp_dir.path().join("context");
        std::fs::create_dir_all(&context_dir)?;

        // Copy ZIP file to context directory
        let target_zip = context_dir.join("amazon-q-developer-cli.zip");
        std::fs::copy(&zip_path, &target_zip)?;

        // Create test script
        let test_script = self.generate_test_script(distro, version, arch, libc);
        std::fs::write(context_dir.join("test-script.sh"), test_script)?;

        // Write Dockerfile to context directory
        std::fs::write(context_dir.join("Dockerfile"), &dockerfile)?;

        // Build image
        let tag = DockerfileGenerator::generate_tag(distro, version, arch, libc);
        let build_opts = BuildImageOptions {
            dockerfile: "Dockerfile".to_string(),
            t: tag.clone(),
            buildargs: HashMap::from([("LIBC_VARIANT".to_string(), libc.to_string())]),
            ..Default::default()
        };

        info!("Building Docker image: {}", tag);

        // Convert the path to a string and then to a static string to satisfy the lifetime requirements
        let context_path = context_dir.to_str().unwrap().to_string();

        let mut build_stream = self.docker.build_image(build_opts, None, Some(context_path.into()));

        while let Some(build_result) = build_stream.next().await {
            match build_result {
                Ok(output) => {
                    if let Some(stream) = output.stream {
                        print!("{}", stream);
                    }
                    if let Some(error) = output.error {
                        error!("Build error: {}", error);
                        return Err(anyhow!("Docker build failed: {}", error));
                    }
                },
                Err(e) => {
                    error!("Build error: {}", e);
                    return Err(anyhow!("Docker build failed: {}", e));
                },
            }
        }

        // Create container
        let container_config = Config {
            image: Some(tag.clone()),
            cmd: Some(vec![
                "/bin/bash".to_string(),
                "/amazon-q-developer-cli/test-script.sh".to_string(),
            ]),
            ..Default::default()
        };

        let container_opts = CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        };

        let container = self
            .docker
            .create_container(Some(container_opts), container_config)
            .await?;

        // Start container
        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await?;

        // Wait for container to exit
        let wait_result = self
            .docker
            .wait_container::<String>(&container.id, None)
            .collect::<Vec<_>>()
            .await;

        // Get container logs
        let logs = self
            .docker
            .logs::<String>(&container.id, None)
            .collect::<Vec<_>>()
            .await;
        for log_result in logs.into_iter().flatten() {
            print!("{}", log_result);
        }

        // Check exit code
        let success = wait_result.iter().all(|result| {
            if let Ok(wait) = result {
                wait.status_code == 0
            } else {
                false
            }
        });

        // Clean up
        if !self.keep_containers {
            let rm_opts = RemoveContainerOptions {
                force: true,
                ..Default::default()
            };

            if let Err(e) = self.docker.remove_container(&container.id, Some(rm_opts)).await {
                warn!("Failed to remove container: {}", e);
            }

            if let Err(e) = self.docker.remove_image(&tag, None, None).await {
                warn!("Failed to remove image: {}", e);
            }
        } else {
            info!("Container kept for inspection: {}", container_name);
        }

        Ok(success)
    }

    /// Generate test script for container
    fn generate_test_script(&self, distro: &str, version: &str, arch: &str, libc: &str) -> String {
        format!(
            r#"#!/bin/bash
set -e

echo "=== Testing Amazon Q Developer CLI installation ==="
echo "Distribution: {}:{}"
echo "Architecture: {}"
echo "Libc: {}"

# Test as root user
echo "=== Testing as root user ==="
cd /amazon-q-developer-cli/bundle
unzip -o amazon-q-developer-cli.zip
ls -la
cd q
./install.sh --force --no-confirm

# Verify installation
which q
q --version

# Test as regular user
echo "=== Testing as regular user ==="
su - quser -c "cd /amazon-q-developer-cli/bundle && unzip -o amazon-q-developer-cli.zip"
su - quser -c "cd /amazon-q-developer-cli/bundle/q && ./install.sh --no-confirm"

# Verify installation
su - quser -c "which q"
su - quser -c "q --version"

echo "=== Test completed successfully ==="
"#,
            distro, version, arch, libc
        )
    }
}
