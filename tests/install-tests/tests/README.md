# Amazon Q Developer CLI Installation Test Files

This directory contains the test files for verifying the installation of Amazon Q Developer CLI across different Linux distributions.

## Test Files Overview

### `test_installation.py`

Tests the installation of Amazon Q Developer CLI using the native package manager (DEB packages for Debian/Ubuntu).

- `test_fresh_installation`: Installs the package on a clean system and verifies it works correctly

### `test_zip_installation.py`

Tests the installation of Amazon Q Developer CLI using ZIP packages.

- `test_zip_installation`: Extracts the ZIP file, runs the included `install.sh` script, and verifies the binary is installed in the root user's home directory

### `test_appimage_installation.py`

Tests the installation of Amazon Q Developer CLI using AppImage packages.

- `test_appimage_installation`: Copies the AppImage to a designated location, makes it executable, and creates a symbolic link

### `test_uninstallation.py`

Tests the uninstallation of Amazon Q Developer CLI that was installed using the native package manager.

- `test_uninstallation`: Uninstalls the package and verifies it was removed correctly

### `test_appimage_uninstallation.py`

Tests the uninstallation of Amazon Q Developer CLI that was installed using AppImage.

- `test_appimage_uninstallation`: Removes the AppImage and symbolic links, then verifies the binary is gone

### `test_musl_requirement.py`

Tests the requirement for musl variants on older distributions.

- `test_musl_requirement`: Verifies that only musl variants work on distributions that require them

### `test_real_installation.py`

Tests installation on the actual system rather than in a container.

- `test_real_installation`: Installs the package on the host system and verifies it works

### `test_real_uninstallation.py`

Tests uninstallation on the actual system rather than in a container.

- `test_real_uninstallation`: Uninstalls the package from the host system and verifies it was removed

## Common Test Structure

All test files follow a similar structure:

1. Import necessary modules
2. Define a `print_output` function for consistent command output display
3. Define test functions with appropriate pytest markers
4. Implement installation/uninstallation logic
5. Verify the installation/uninstallation was successful
6. Record test results in JSON format

## Adding New Tests

When adding new tests:

1. Follow the existing pattern for consistency
2. Use the `print_output` function for all command outputs
3. Add appropriate pytest markers
4. Include proper error handling and result recording
5. Document the test in this README
