# Amazon Q Developer CLI Installation Tests

This directory contains tests for verifying the installation of Amazon Q Developer CLI across different Linux distributions.

## Test Overview

The tests verify that:

1. The Amazon Q Developer CLI package can be installed on various Linux distributions
2. The package correctly specifies its dependencies
3. The binary is properly installed and executable
4. The binary can be uninstalled cleanly

## Running Tests

To run the tests:

```bash
# Run all tests
python -m pytest

# Run ZIP installation test for x86_64 with specific libc variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version 22.04 --architecture x86_64 --libc-variant musl

# Run ZIP installation test for arm64
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version 22.04 --architecture arm64 --libc-variant glibc
```

## Test Results

Test results are stored in the `results/` directory in JSON format. Each result file contains:

- Distribution name and version
- Architecture
- C library variant (musl or glibc)
- Test name
- Timestamp
- Status (pass/fail)
- Execution time
- Error message (if applicable)

A summary report is automatically generated after all tests have completed and is available at `results/zip_test_summary.json`.

## Installation Methods

### ZIP Installation

The ZIP installation method:

1. Extracts the ZIP file to a temporary location
2. Runs the included `install.sh` script from the `q` subdirectory
3. Installs the binary to the user's home directory (typically `~/.local/bin`)
4. Updates the user's profile files to add the binary to PATH

Two variants of ZIP packages are available:

- Standard (glibc): For modern distributions with GNU libc
- Musl: For Alpine Linux and other distributions using musl libc

To run the ZIP installation test with a specific variant:

```bash
# Test musl variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version 22.04 --architecture x86_64 --libc-variant musl

# Test glibc variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version 22.04 --architecture x86_64 --libc-variant glibc
```

### Native Package Manager (DEB) - TODO

For Debian-based distributions, the package requires:

1. Update package repositories first:
   ```bash
   apt-get update
   ```

2. Install the package:
   ```bash
   apt-get install -y ./amazon-q-developer-cli_amd64.deb
   ```

### AppImage Installation - TODO

The AppImage installation method:

1. Copies the AppImage to a designated location
2. Makes the AppImage executable
3. Creates a symbolic link in `/usr/local/bin/q`

## Container Runtimes

The tests support multiple container runtimes:

- Docker
- Finch
- Podman

You can specify which runtime to use with the `--runtime` option:

```bash
python -m pytest tests/test_zip_installation.py::test_zip_installation --runtime finch --distribution ubuntu --dist-version 22.04 --architecture x86_64 --libc-variant glibc
```

If no runtime is specified, the test framework will automatically detect an available runtime.

## Troubleshooting

If installation fails, check:

1. Package repositories are up-to-date
2. Required dependencies are available
3. The package is compatible with the distribution version
4. There is sufficient disk space

Common issues:

- Missing dependencies: Run `apt-get install -f` to fix dependency issues
- Binary not in PATH: Check the installation method
- Permission issues: Ensure the binary has executable permissions
- Container runtime not found: Install Docker, Finch, or Podman

## Development

When adding new tests or modifying existing ones:

1. Update the test fixtures in `conftest.py` if needed
2. Add any new test functions to the appropriate test file
3. Update the known failures list if a test is expected to fail on certain distributions

### Test Matrix

The test matrix is defined in `conftest.py` and includes:

- Distributions: Ubuntu, Debian, Fedora, Amazon Linux, Rocky Linux, Alpine
- Versions: Various versions for each distribution
- Architectures: x86_64, arm64
- Libc variants: musl, glibc

The matrix is automatically filtered to only include valid combinations (e.g., glibc variant is only tested on distributions with a sufficient glibc version).

### Important Note About ZIP Structure

The ZIP file extracts into a `q` subdirectory that contains the installation files. The test has been updated to handle this structure correctly by:

1. Extracting the ZIP file to a temporary directory
2. Running the `install.sh` script from the `q` subdirectory
3. Verifying that the binary is installed correctly

This is intentional behavior to keep all installation files contained and organized.
