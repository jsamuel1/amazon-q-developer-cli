# ZIP Installation Tests

This crate provides integration tests for the Amazon Q Developer CLI installation process across various Linux distributions.

## Overview

The tests verify that the installation script (`install.sh`) included in the ZIP distribution works correctly on different Linux distributions, architectures, and libc variants. The tests run in Docker/Podman/Finch containers to simulate different environments.

## Running Tests

### Prerequisites

- Docker, Podman, or Finch installed and running
- ZIP files in the `test_data` directory
- Rust toolchain installed

### Using Cargo Test

Run all tests:

```bash
cargo test
```

Run specific root or user installation tests:

```bash
# Run root installation tests
cargo test test_ubuntu_latest_root

# Run user installation tests
cargo test test_ubuntu_latest_user

# Run all root installation tests
cargo test test_all_distributions_root

# Run all user installation tests
cargo test test_all_distributions_user
```

Run with output:

```bash
cargo test -- --nocapture
```

### Using the Run Script

For convenience, you can also use the provided run script:

```bash
./run-test.sh [distribution] [version] [architecture] [libc] [test_type]
```

Where `test_type` can be `root`, `user`, or `both` (default).

Example:
```bash
# Run both root and user tests
./run-test.sh ubuntu 22.04 x86_64 glibc both

# Run only root test
./run-test.sh ubuntu 22.04 x86_64 glibc root

# Run only user test
./run-test.sh ubuntu 22.04 x86_64 glibc user
```

## Test Data

Place the ZIP files in the `test_data` directory with the following naming convention:

- `amazon-q-developer-cli-x86_64-linux.zip` - For x86_64 glibc builds
- `amazon-q-developer-cli-aarch64-linux.zip` - For aarch64 glibc builds
- `amazon-q-developer-cli-x86_64-linux-musl.zip` - For x86_64 musl builds
- `amazon-q-developer-cli-aarch64-linux-musl.zip` - For aarch64 musl builds

## Test Types

The tests are split into two separate types:

### Root Installation Tests

These tests verify that the CLI can be installed as root. The tests:
- Set the `Q_INSTALL_ROOT=true` environment variable
- Run the installer with `--force --no-confirm` flags
- Verify the installation works for the root user

### User Installation Tests

These tests verify that the CLI can be installed as a regular user. The tests:
- Run the installer as a non-root user with `--no-confirm` flag
- Verify the installation works for the regular user

## Supported Distributions

The tests currently support the following distributions:

- Ubuntu: 24.04, 22.04, 20.04
- Debian: 12, 11
- Amazon Linux: 2023, 2
- Alpine: 3.19, 3.18
- Fedora: 39, 38
- Rocky Linux: 9, 8

## Adding New Distributions

To add a new distribution for testing:

1. Update the `get_distributions` function in `src/lib.rs`
2. Add any distribution-specific logic to the `DockerfileGenerator` in `src/dockerfile.rs`
3. Add tests for the new distribution in `tests/integration_test.rs`

## Integration with Workspace

This crate is part of the Amazon Q Developer CLI workspace and is designed to be run as part of the CI/CD pipeline to verify installation across different platforms.
