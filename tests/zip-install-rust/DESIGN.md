# Amazon Q Developer CLI ZIP Installation Tests - Design Document

## Overview

This document describes the design and implementation of the Rust-based ZIP installation tests for the Amazon Q Developer CLI.

## Goals

1. Replace the existing Node.js-based ZIP installation tests with a Rust implementation
2. Generate Dockerfiles dynamically instead of checking them into source control
3. Support both Docker and Finch container runtimes
4. Work on ARM64-based Mac instances (with Rosetta) and CodeBuild servers
5. Match the style and dependencies of other Rust tests in the project
6. Provide both CLI and integration test interfaces

## Architecture

The implementation consists of several key components:

### 1. Configuration (`config.rs`)

Defines the supported distributions, architectures, and libc variants. This includes:
- Distribution configurations (name, version, architectures, libc variants)
- Distribution-specific parameters (base image, package manager, etc.)
- Common and distribution-specific packages

### 2. Dockerfile Generation (`dockerfile.rs`)

Dynamically generates Dockerfiles for each distribution/version/architecture combination:
- Creates a Dockerfile with the appropriate base image
- Sets up the environment and installs required packages
- Creates a test user and configures sudo access
- Handles SELinux and other distribution-specific setup

### 3. Container Management (`container.rs`)

Manages Docker/Finch containers for testing:
- Builds container images using the generated Dockerfiles
- Runs containers with the ZIP file mounted
- Executes commands in the containers
- Cleans up containers after testing

### 4. Test Runner (`runner.rs`)

Orchestrates the testing process:
- Finds the appropriate ZIP file for each architecture/libc variant
- Builds and runs containers
- Executes test cases in the containers
- Reports test results

### 5. Utilities (`utils.rs`)

Provides helper functions:
- Finding ZIP files
- Parsing ZIP filenames
- Checking for available commands
- Detecting container runtime (Docker or Finch)

### 6. CLI Interface (`main.rs`)

Provides a command-line interface for running tests:
- `test-all`: Run tests for all distributions
- `test`: Run tests for a specific distribution
- `generate-dockerfile`: Generate a Dockerfile for a specific distribution

### 7. Integration Tests (`tests/integration_test.rs`)

Provides integration tests that can be run with `cargo test`:
- Tests for specific distributions
- Tests for Dockerfile generation
- Tests for tag generation

## Workflow

1. The user provides a directory containing ZIP files
2. The test runner finds the appropriate ZIP file for each architecture/libc variant
3. For each distribution/version/architecture/libc combination:
   a. Generate a Dockerfile
   b. Build a container image
   c. Run a container with the ZIP file mounted
   d. Execute test cases in the container
   e. Report test results
   f. Clean up the container

## Advantages Over Previous Implementation

1. **No Checked-in Dockerfiles**: Dockerfiles are generated dynamically, reducing repository size and making it easier to add new distributions.

2. **Rust Integration**: Fully integrated with the Rust ecosystem, using the same workspace dependencies as the main project.

3. **Container Runtime Support**: Automatically detects and uses either Docker or Finch, making it compatible with more environments.

4. **Efficient Resource Usage**: Uses async/await for efficient container operations, reducing resource usage.

5. **Better Error Handling**: Uses Rust's error handling to provide more detailed error messages.

6. **Integration Tests**: Provides integration tests that can be run with `cargo test`.

## Implementation Details

### Container Image Design

Each container image is built with:
- A base image for the specific distribution/version
- Common packages (unzip, sudo, curl)
- Distribution-specific packages
- A test user with sudo access
- A directory for the ZIP installer

### Test Cases

The tests verify that:
1. The ZIP file can be extracted
2. The installer works when run as root
3. The installer works when run as a regular user
4. The installed `q` command functions correctly

### Error Handling

The implementation uses Rust's `anyhow` and `thiserror` crates for error handling:
- `anyhow` for general error handling
- `thiserror` for defining specific error types
- Context is added to errors to provide more detailed error messages

### Logging

The implementation uses the `log` crate for logging:
- `info` for general information
- `debug` for detailed debugging information
- `error` for error messages

## Future Improvements

1. **Parallel Testing**: Run tests for multiple distributions in parallel to reduce test time.

2. **Test Result Reporting**: Improve test result reporting with more detailed information.

3. **Test Coverage**: Add more test cases to cover more scenarios.

4. **CI Integration**: Integrate with CI/CD pipelines for automated testing.
