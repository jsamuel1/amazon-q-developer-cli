# Amazon Q Developer CLI Installation Tests

This document outlines the requirements and approach for testing the installation of Amazon Q Developer CLI packages across various Linux distributions and architectures.

## Objectives

1. Verify that Amazon Q Developer CLI packages install correctly across supported Linux distributions
2. Test installation on multiple CPU architectures (x86_64, aarch64)
3. Ensure compatibility with different versions of each distribution (oldest supported, current LTS, latest)
4. Generate standardized test results for analysis and reporting
5. Automate the testing process using containers for reproducibility

## Test Matrix

### Architectures

- x86_64 (AMD64)
- aarch64 (ARM64)

### Linux Distributions and Versions

#### Debian-based

- Debian
  - Oldest supported: Debian 10 (Buster)
  - Current LTS: Debian 11 (Bullseye)
  - Latest: Debian 12 (Bookworm)
- Ubuntu
  - Oldest supported: Ubuntu 20.04 LTS (Focal Fossa)
  - Current LTS: Ubuntu 22.04 LTS (Jammy Jellyfish)
  - Latest: Ubuntu 23.10 (Mantic Minotaur)
  - Generic LTS: ubuntu:lts (always points to the current LTS release)

#### Other

- Amazon Linux

- Fedora
  - Oldest supported: Fedora 36
  - Current: Fedora 38
  - Latest: Fedora 39
- RHEL/CentOS/Rocky Linux
  - Oldest supported: RHEL/CentOS 8
  - Current: RHEL/CentOS/Rocky Linux 9
- Arch Linux (rolling release, latest only)

## Test Scenarios

For each distribution and architecture combination, the following scenarios will be tested:

1. **Fresh Installation**

   - Install Amazon Q Developer CLI on a clean system
   - Verify installation success
   - Check binary availability and execution

2. **Upgrade Installation**

   - Install an older version of Amazon Q Developer CLI
   - Upgrade to the current version
   - Verify upgrade success

3. **Dependencies**
   - Verify all dependencies are correctly installed
   - Test with minimal system configurations

## Implementation Approach

### Container-based Testing

- Use Docker/Podman containers to simulate each distribution and version
- Create base images for each distribution/version/architecture combination
- Use multi-stage builds where appropriate to optimize testing
- Implement container orchestration for parallel test execution

### Test Framework

- Implement tests using Python's pytest framework
- Create a standardized test result format
- Generate JUnit XML reports for CI/CD integration
- Implement logging for detailed test execution information

### Test Execution

1. Build or pull container images for each distribution/version/architecture
2. Execute installation tests within containers
3. Capture test results and logs
4. Generate consolidated test reports

## Test Result Format

Test results will be reported in a standardized format containing:

```json
{
  "test_id": "unique-test-identifier",
  "distribution": "ubuntu",
  "version": "22.04",
  "architecture": "x86_64",
  "scenario": "fresh-install",
  "package_version": "1.0.0",
  "status": "pass|fail",
  "execution_time": 12.5,
  "timestamp": "2025-03-15T01:10:53Z",
  "error_details": "Error message if failed",
  "logs": "path/to/detailed/logs"
}
```

## Special Considerations

### Binary Variants

- For older distributions, only the `-musl` binary variant is expected to work
- Tests should verify that:
  - The `-musl` variant installs and runs correctly on older distributions
  - The regular variant fails appropriately on unsupported older distributions
  - The appropriate variant is selected based on the distribution and version

### Architecture Support

- Not all distributions support all architectures
- Tests should handle cases where certain distribution/architecture combinations are not supported

## Package Formats

The Amazon Q Developer CLI is distributed in multiple package formats:

1. **Debian (.deb) packages**

   - Used for Debian and Ubuntu distributions
   - Available in standard and -musl variants for older distributions

2. **AppImage (.AppImage) packages**

   - Universal Linux package format that works across distributions
   - Primary installation method for non-Debian distributions
   - Alternative installation method for Debian-based distributions
   - Self-contained and does not require root privileges

3. Zip files (.zip) packages
   - Used for all derivatives as a minimal installations

## Installation Methods

The test framework will verify installation using the following methods:

1. **Native Package Manager Installation**

   - apt/apt-get for Debian-based distributions

2. **AppImage Installation**

   - Primary method for non-Debian distributions
   - Alternative method for Debian-based distributions
   - Tests will verify both installation methods on Debian-based systems

3. Zip Installation
   - Minimal install for any distribution

## Supported Distributions

The installation tests cover the following Linux distributions:

1. **Debian-based**:

   - Debian 10, 11, 12
   - Ubuntu 20.04, 22.04, 23.10, LTS

2. **RPM-based**:

   - Fedora 36, 38, 39
   - Rocky Linux 8, 9
   - Amazon Linux 2, 2023

3. **Alpine Linux**:

   - Alpine 3.16, 3.17, 3.18, latest

4. **Arch Linux**:
   - Latest rolling release
