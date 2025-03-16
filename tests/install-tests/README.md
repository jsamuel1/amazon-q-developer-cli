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

````bash
# Run all tests
python -m pytest

# Run ZIP installation test for x86_64 with auto-detected libc variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version lts --architecture x86_64

# Run ZIP installation test for x86_64 with specific libc variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version lts --architecture x86_64 --libc-variant musl

# Run ZIP installation test for arm64
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version lts --architecture arm64

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

## Installation Methods

### Native Package Manager (DEB) - TODO

For Debian-based distributions, the package requires:

1. Update package repositories first:
   ```bash
   apt-get update
````

2. Install the package:

   ```bash
   apt-get install -y ./amazon-q-developer-cli_amd64.deb
   ```

The package will automatically install all required dependencies.

### ZIP Installation

The ZIP installation method:

1. Extracts the ZIP file to a temporary location
2. Runs the included `install.sh` script
3. Installs the binary to the user's home directory (typically `~/.amazon-q/bin`)
4. Updates the user's profile files to add the binary to PATH

Two variants of ZIP packages are available:

- Standard (glibc): For modern distributions with GNU libc
- Musl: For Alpine Linux and other distributions using musl libc

To run the ZIP installation test with a specific variant:

```bash
# Test musl variant
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version lts --architecture x86_64 --libc-variant musl

# Test glibc variant (default)
python -m pytest tests/test_zip_installation.py::test_zip_installation --distribution ubuntu --dist-version lts --architecture x86_64 --libc-variant glibc
```

### AppImage Installation - TODO

The AppImage installation method:

1. Copies the AppImage to a designated location (e.g., `/opt/amazon-q-cli/`)
2. Makes the AppImage executable
3. Creates a symbolic link in `/usr/local/bin/q`

## Container Runtimes

The tests support multiple container runtimes:

- Docker
- Finch
- Podman

You can specify which runtime to use with the `--runtime` option:

```bash
python -m pytest tests/test_zip_installation.py::test_zip_installation --runtime finch
```

If no runtime is specified, the test framework will automatically detect an available runtime in the following order:

1. Docker
2. Finch
3. Podman

## Troubleshooting

If installation fails, check:

1. Package repositories are up-to-date
2. Required dependencies are available
3. The package is compatible with the distribution version
4. There is sufficient disk space

Common issues:

- Missing dependencies: Run `apt-get install -f` to fix dependency issues
- Binary not in PATH: Check the installation method:
  - DEB package: The binary should be installed at `/usr/bin/q`
  - ZIP package: The binary should be in `~/.amazon-q/bin/q`
  - AppImage: The binary should be linked at `/usr/local/bin/q`
- Permission issues: Ensure the binary has executable permissions
- Container runtime not found: Install Docker, Finch, or Podman

## Development

When adding new tests or modifying existing ones:

1. Update the test fixtures in `conftest.py` if needed
2. Add any new test functions to the appropriate test file
3. Update the known failures list if a test is expected to fail on certain distributions

### Adding a New Installation Method

To add a new installation method:

1. Create a new test file (e.g., `test_new_method_installation.py`)
2. Implement the installation logic
3. Add appropriate assertions to verify the installation
4. Add an uninstallation test if applicable
5. Update this README with documentation for the new method

### Test Output

All tests use the `print_output` function to display command outputs consistently:

```python
def print_output(command, exit_code, output):
    """Helper function to print command output consistently."""
    if isinstance(output, bytes):
        output = output.decode('utf-8', errors='replace')
    print(f"\n=== Command: {command} ===")
    print(f"Exit code: {exit_code}")
    print(f"Output:\n{output}")
    print("=" * 50)
```

This ensures that all command outputs are properly displayed with context about which command was run and what its exit code was.

## Running All ZIP Installation Tests

To run all variants of the ZIP installation tests across different distributions, architectures, and libc variants, use the `run_zip_tests.py` script:

```bash
# Run all tests with default settings
./run_zip_tests.py

# Run with a specific container runtime
./run_zip_tests.py --runtime finch

# Run for specific distributions only
./run_zip_tests.py --distributions ubuntu:22.04,debian:11

# Run for specific architectures only
./run_zip_tests.py --architectures x86_64

# Run for specific libc variants only
./run_zip_tests.py --libc-variants musl

# Combine options
./run_zip_tests.py --runtime finch --distributions ubuntu:22.04 --architectures x86_64 --libc-variants musl

# Skip summary generation
./run_zip_tests.py --no-summary
```

The script will:

1. Run each test combination sequentially
2. Display the output of each test
3. Generate a summary report at the end (unless --no-summary is specified)
4. Save detailed results in the `results/` directory

This is the easiest way to verify that all ZIP installation variants work correctly across your supported platforms.
