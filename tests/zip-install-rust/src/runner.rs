use anyhow::{Context, Result};
use log::{error, info};
use std::path::Path;

use crate::container::ContainerManager;
use crate::container::ContainerOptions;
use crate::finch_container::FinchContainerManager;
use crate::utils;

/// Test runner for ZIP installation tests
pub struct TestRunner {
    keep_containers: bool,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new(keep_containers: bool) -> Self {
        Self { keep_containers }
    }

    /// Run a test for the specified options
    pub async fn run_test(
        &self,
        distro: &str,
        version: &str,
        arch: &str,
        libc: &str,
        zip_dir: &Path,
    ) -> Result<()> {
        info!("Running test for {}-{}-{}-{}", distro, version, arch, libc);

        // Find the appropriate ZIP file
        let zip_files = utils::find_zip_files(zip_dir).await?;
        let mut zip_path = None;

        for file in zip_files {
            if let Some((file_arch, file_libc)) = utils::parse_zip_filename(&file) {
                if file_arch == arch && file_libc == libc {
                    zip_path = Some(file);
                    break;
                }
            }
        }

        let zip_path = zip_path.ok_or_else(|| {
            anyhow::anyhow!(
                "No ZIP file found for architecture {} and libc {}",
                arch,
                libc
            )
        })?;

        info!("Using ZIP file: {}", zip_path.display());

        // Create container options
        let options = ContainerOptions {
            distro: distro.to_string(),
            version: version.to_string(),
            architecture: arch.to_string(),
            libc_variant: libc.to_string(),
            zip_path: zip_path.clone(),
        };

        // Determine which container runtime to use
        let runtime = utils::check_container_runtime()?;
        info!("Using container runtime: {}", runtime);

        if runtime == "finch" {
            self.run_test_with_finch(&options).await
        } else {
            self.run_test_with_docker(&options).await
        }
    }

    /// Run a test using Docker
    async fn run_test_with_docker(&self, options: &ContainerOptions) -> Result<()> {
        // Create container manager
        let container_manager = ContainerManager::new()
            .await
            .context("Failed to create container manager")?;

        // Build image
        let tag = container_manager
            .build_image(options)
            .await
            .context("Failed to build image")?;

        // Run container
        let container_id = container_manager
            .run_container(&tag, &options.zip_path, options)
            .await
            .context("Failed to run container")?;

        // Execute test script
        let (exit_code, output) = container_manager
            .exec_command(&container_id, &["/bin/bash", "/amazon-q-developer-cli/test-script.sh"])
            .await
            .context("Failed to execute test script")?;

        // Check exit code
        if exit_code != 0 {
            error!("Test failed with exit code {}", exit_code);
            error!("Output: {}", output);
            return Err(anyhow::anyhow!("Test failed with exit code {}", exit_code));
        }

        // Clean up container if needed
        if !self.keep_containers {
            container_manager
                .cleanup_container(&container_id)
                .await
                .context("Failed to clean up container")?;
        }

        Ok(())
    }

    /// Run a test using Finch
    async fn run_test_with_finch(&self, options: &ContainerOptions) -> Result<()> {
        // Create container manager
        let container_manager = FinchContainerManager::new()
            .await
            .context("Failed to create Finch container manager")?;

        // Build image
        let tag = container_manager
            .build_image(options)
            .await
            .context("Failed to build image")?;

        // Run container
        let container_id = container_manager
            .run_container(&tag, options)
            .await
            .context("Failed to run container")?;

        // Clean up container if needed
        if !self.keep_containers {
            container_manager
                .cleanup_container(&container_id)
                .await
                .context("Failed to clean up container")?;
        }

        Ok(())
    }
}
