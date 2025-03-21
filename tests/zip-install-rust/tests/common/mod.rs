use std::path::PathBuf;

/// Get the path to the test data directory
pub fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data")
}

/// Check if Docker is available for tests
pub fn docker_available() -> bool {
    std::process::Command::new("docker").arg("--version").output().is_ok()
}

/// Check if Finch is available for tests
pub fn finch_available() -> bool {
    std::process::Command::new("finch").arg("--version").output().is_ok()
}

/// Check if Podman is available for tests
pub fn podman_available() -> bool {
    std::process::Command::new("podman").arg("--version").output().is_ok()
}

/// Check if any container runtime is available
pub fn container_runtime_available() -> bool {
    docker_available() || finch_available() || podman_available()
}

/// Check if test data is available
pub fn test_data_available() -> bool {
    let test_dir = test_data_dir();
    test_dir.exists()
        && test_dir.is_dir()
        && (test_dir.join("amazon-q-developer-cli-x86_64-linux.zip").exists()
            || test_dir.join("amazon-q-developer-cli-aarch64-linux.zip").exists())
}
