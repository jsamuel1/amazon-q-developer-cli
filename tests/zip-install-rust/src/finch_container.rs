use anyhow::{Context, Result};
use log::{debug, info};
use std::process::Output;
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;

use crate::container::ContainerOptions;
use crate::dockerfile::DockerfileGenerator;

/// Manager for Finch containers
pub struct FinchContainerManager {
    temp_dir: TempDir,
}

impl FinchContainerManager {
    /// Create a new container manager
    pub async fn new() -> Result<Self> {
        // Check if Finch is available
        let output = Command::new("finch")
            .arg("--version")
            .output()
            .await
            .context("Failed to execute finch command")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Finch is not available. Please install Finch."
            ));
        }

        let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

        Ok(Self { temp_dir })
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

        // Create a test script
        let test_script_path = self.temp_dir.path().join("test-script.sh");
        let test_script_content = self.generate_test_script(&options);
        fs::write(&test_script_path, test_script_content)
            .await
            .context("Failed to write test script")?;

        // Make the test script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&test_script_path)
                .await
                .context("Failed to get test script metadata")?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&test_script_path, perms)
                .await
                .context("Failed to set test script permissions")?;
        }

        // Copy the ZIP file to the temp directory
        let zip_dest_path = self.temp_dir.path().join("amazon-q-developer-cli.zip");
        fs::copy(&options.zip_path, &zip_dest_path)
            .await
            .context("Failed to copy ZIP file")?;

        // Determine platform flag based on architecture
        let platform_flag = if cfg!(target_arch = "aarch64") {
            // For ARM64 Mac, we need to use --platform=linux/amd64 for x86_64 images
            if options.architecture == "x86_64" {
                Some("--platform=linux/amd64")
            } else {
                Some("--platform=linux/arm64")
            }
        } else {
            // For Intel Mac, we need to use --platform=linux/arm64 for aarch64 images
            if options.architecture == "aarch64" {
                Some("--platform=linux/arm64")
            } else {
                Some("--platform=linux/amd64")
            }
        };

        // Build the image using finch
        let mut build_cmd = Command::new("finch");
        build_cmd.arg("build");

        if let Some(platform) = platform_flag {
            build_cmd.arg(platform);
        }

        build_cmd
            .arg("-t")
            .arg(&tag)
            .arg("-f")
            .arg(dockerfile_path)
            .current_dir(self.temp_dir.path());

        debug!("Running build command: {:?}", build_cmd);

        let output = build_cmd
            .output()
            .await
            .context("Failed to execute finch build command")?;

        self.log_command_output(&output);

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to build image"));
        }

        Ok(tag)
    }

    /// Run a container with the specified image tag
    pub async fn run_container(&self, tag: &str, options: &ContainerOptions) -> Result<String> {
        info!("Running container: {}", tag);

        // Create a container name with test details and timestamp
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

        // Run the container using finch
        let output = Command::new("finch")
            .arg("run")
            .arg("--name")
            .arg(&container_name)
            .arg(tag)
            .output()
            .await
            .context("Failed to execute finch run command")?;

        self.log_command_output(&output);

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to run container"));
        }

        Ok(container_name)
    }

    /// Clean up a container
    pub async fn cleanup_container(&self, container_id: &str) -> Result<()> {
        info!("Cleaning up container: {}", container_id);

        // Remove the container
        let output = Command::new("finch")
            .arg("rm")
            .arg("-f")
            .arg(container_id)
            .output()
            .await
            .context("Failed to execute finch rm command")?;

        self.log_command_output(&output);

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to remove container"));
        }

        // Remove the image
        let output = Command::new("finch")
            .arg("rmi")
            .arg("-f")
            .arg(container_id)
            .output()
            .await
            .context("Failed to execute finch rmi command")?;

        self.log_command_output(&output);

        // We don't care if this fails, as the image might be used by other containers

        Ok(())
    }

    /// Generate a test script for the container
    fn generate_test_script(&self, options: &ContainerOptions) -> String {
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
            options.distro, options.version, options.architecture, options.libc_variant
        )
    }

    /// Log command output
    fn log_command_output(&self, output: &Output) {
        if !output.stdout.is_empty() {
            debug!("Command stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            debug!("Command stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
