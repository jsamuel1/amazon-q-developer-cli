# ZIP Installation Tests

This crate provides integration tests for the ZIP installation process of Amazon Q Developer CLI.

## Overview

The tests verify that the installation script (`install.sh`) included in the ZIP distribution works correctly on different Linux distributions, architectures, and libc variants. The tests run in Docker/Podman/Finch containers to simulate different environments.

## Running Tests

### Prerequisites

- Docker, Podman, or Finch installed and running
- ZIP files in the `test_data` directory
- Rust toolchain installed

### Using Cargo Test

Run all tests

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
