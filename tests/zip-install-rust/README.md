# ZIP Installation Tests

This crate provides integration tests for the Amazon Q Developer CLI installation process across various Linux distributions.

## Overview

The tests verify that the installation script (`install.sh`) included in the ZIP distribution works correctly on different Linux distributions, architectures, and libc variants.

## Running Tests

### Prerequisites

- Docker, Podman, or Finch installed and running
- ZIP files in the `test_data` directory

### Using Cargo Test

Run all tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test test_ubuntu_latest
```

Run with output:

```bash
cargo test -- --nocapture
```

## Test Data

Place the ZIP files in the `test_data` directory with the following naming convention:

- `amazon-q-developer-cli-x86_64-linux.zip` - For x86_64 glibc builds
- `amazon-q-developer-cli-aarch64-linux.zip` - For aarch64 glibc builds
- `amazon-q-developer-cli-x86_64-linux-musl.zip` - For x86_64 musl builds
- `amazon-q-developer-cli-aarch64-linux-musl.zip` - For aarch64 musl builds

## Adding New Distributions

To add a new distribution for testing, update the `get_distributions` function in `src/lib.rs`.

## Integration with Workspace

This crate is part of the Amazon Q Developer CLI workspace and is designed to be run as part of the CI/CD pipeline to verify installation across different platforms.
