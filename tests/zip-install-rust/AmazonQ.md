# Amazon Q Developer CLI ZIP Installation Tests

This document provides information about the Amazon Q Developer CLI ZIP installation tests.

## Overview

The Amazon Q Developer CLI ZIP installation tests verify that the installation script (`install.sh`) included in the ZIP distribution works correctly across various Linux distributions, architectures, and libc variants.

## Supported Environments

The tests support the following environments:

### Distributions
- Ubuntu: 24.04, 22.04, 20.04
- Debian: 12, 11
- Amazon Linux: 2023, 2
- Alpine: 3.19, 3.18
- Fedora: 39, 38
- Rocky Linux: 9, 8

### Architectures
- x86_64
- aarch64 (ARM64)

### Libc Variants
- glibc
- musl

## Test Process

The tests are split into two separate test types:

### Root Installation Tests

1. Create a Docker/Podman/Finch container for the target environment
2. Copy the appropriate ZIP file into the container
3. Extract the ZIP file
4. Set the `Q_INSTALL_ROOT=true` environment variable
5. Run the installation script as root with `--force --no-confirm` flags
6. Verify that the `q` command is installed and works for the root user

### User Installation Tests

1. Create a Docker/Podman/Finch container for the target environment
2. Copy the appropriate ZIP file into the container
3. Extract the ZIP file as a regular user
4. Run the installation script as a regular user with `--no-confirm` flag
5. Verify that the `q` command is installed and works for the regular user

## Running Tests

You can run the tests using either:

1. Cargo test:
   ```bash
   # Run all tests
   cargo test
   
   # Run only root installation tests
   cargo test test_all_distributions_root
   
   # Run only user installation tests
   cargo test test_all_distributions_user
   
   # Run specific distribution root test
   cargo test test_ubuntu_latest_root
   
   # Run specific distribution user test
   cargo test test_ubuntu_latest_user
   ```

2. The run script:
   ```bash
   # Run both root and user tests
   ./run-test.sh [distribution] [version] [architecture] [libc] both
   
   # Run only root test
   ./run-test.sh [distribution] [version] [architecture] [libc] root
   
   # Run only user test
   ./run-test.sh [distribution] [version] [architecture] [libc] user
   ```

3. The CLI:
   ```bash
   # Run both root and user tests
   cargo run --bin run-test -- --distro [distribution] --version [version] --arch [architecture] --libc [libc] --test-type both
   
   # Run only root test
   cargo run --bin run-test -- --distro [distribution] --version [version] --arch [architecture] --libc [libc] --test-type root
   
   # Run only user test
   cargo run --bin run-test -- --distro [distribution] --version [version] --arch [architecture] --libc [libc] --test-type user
   ```

## Adding New Distributions

To add a new distribution for testing:

1. Update the `get_distributions` function in `src/lib.rs`
2. Add any distribution-specific logic to the `DockerfileGenerator` in `src/dockerfile.rs`
3. Add tests for the new distribution in `tests/integration_test.rs`

## Troubleshooting

If tests fail, check:

1. Docker/Podman/Finch is running
2. The appropriate ZIP files are in the `test_data` directory
3. The distribution is supported
4. The architecture is supported
5. The libc variant is supported
