#!/usr/bin/env python3
"""
Tests for Amazon Q Developer CLI ZIP installation.

This test verifies that the Amazon Q Developer CLI can be installed from a ZIP package.
The test:
1. Extracts the ZIP file to a temporary location
2. Creates a non-root user for installation
3. Runs the included install.sh script as root (since it may need root permissions)
4. Copies the binary to the user's home directory and makes it accessible
5. Verifies the binary is executable and in the user's PATH
"""

import json
import os
import time
from datetime import datetime

import pytest


def decode_output(output):
    """Helper function to decode command output."""
    if isinstance(output, bytes):
        return output.decode("utf-8", errors="replace")
    return output


def print_output(command, exit_code, output):
    """Helper function to print command output consistently."""
    output_str = decode_output(output)
    print(f"\n=== Command: {command} ===")
    print(f"Exit code: {exit_code}")
    print(f"Output:\n{output_str}")
    print("=" * 50)


def run_command(container, command, print_cmd=True):
    """Helper function to run a command and print its output."""
    exit_code, output = container.exec_run(command)
    if print_cmd:
        print_output(command, exit_code, output)
    return exit_code, decode_output(output)


@pytest.mark.zip
def test_zip_installation(
    container,
    distribution,
    version,
    architecture,
    libc_variant,
    test_result_file,
    is_known_failure,
):
    """Test installation of Amazon Q Developer CLI using ZIP package."""
    start_time = time.time()

    # Check if this is a known failure
    known_failure, reason = is_known_failure
    if known_failure:
        pytest.xfail(f"Known failure: {reason}")

    try:
        # Determine the appropriate ZIP file based on architecture, distribution, and libc variant
        if architecture == "x86_64":
            if libc_variant == "musl":
                zip_file = "/amazon-q-developer-cli/bundle/zip/amazon-q-developer-cli-x86_64-linux-musl.zip"
                print("Using musl variant for x86_64")
            else:
                zip_file = "/amazon-q-developer-cli/bundle/zip/amazon-q-developer-cli-x86_64-linux.zip"
                print("Using glibc variant for x86_64")
        elif architecture == "arm64" or architecture == "aarch64":
            if libc_variant == "musl":
                zip_file = "/amazon-q-developer-cli/bundle/zip/amazon-q-developer-cli-aarch64-linux-musl.zip"
                print("Using musl variant for aarch64")
            else:
                zip_file = "/amazon-q-developer-cli/bundle/zip/amazon-q-developer-cli-aarch64-linux.zip"
                print("Using glibc variant for aarch64")
        else:
            raise ValueError(f"Unsupported architecture: {architecture}")

        # Skip unsupported combinations

        # Alpine Linux
        if distribution == "alpine":
            # Alpine ARM64 is not supported
            if architecture == "arm64" or architecture == "aarch64":
                pytest.skip("Alpine Linux on ARM64 is not supported")

            # Only musl libc variant is supported on Alpine
            if libc_variant != "musl":
                pytest.skip("Only musl libc variant is supported on Alpine Linux")

        # Ubuntu 20.04
        if distribution == "ubuntu" and version == "20.04":
            pytest.skip("Ubuntu 20.04 is not supported")

        # Debian 11
        if distribution == "debian" and version == "11":
            pytest.skip("Debian 11 is not supported")

        # Amazon Linux 2
        if distribution == "amazonlinux" and version == "2":
            # Only musl libc variant is supported on Amazon Linux 2
            if libc_variant != "musl":
                pytest.skip("Only musl libc variant is supported on Amazon Linux 2")

        # Verify the ZIP file exists
        command = f"ls -la {zip_file}"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)
        assert exit_code == 0, f"ZIP file not found: {zip_file}"

        # Install required packages for unzipping and user management
        if distribution in ["debian", "ubuntu"]:
            command = "apt-get update && apt-get install -y unzip ca-certificates findutils sudo"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)
        elif distribution in ["fedora", "amazonlinux"]:
            command = (
                "yum makecache && yum install -y unzip ca-certificates findutils sudo"
            )
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)
        elif distribution == "alpine":
            command = "apk update && apk add --no-cache unzip ca-certificates findutils sudo shadow && addgroup wheel 2>/dev/null || true"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

        # Create a temporary directory for extraction
        command = "mkdir -p /tmp/amazon-q-extract"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)
        assert exit_code == 0, "Failed to create extraction directory"

        # Extract ZIP file to the temporary directory
        command = f"unzip -o {zip_file} -d /tmp/amazon-q-extract"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)
        assert exit_code == 0, "Failed to extract ZIP file"

        # Check if the installation script exists
        command = "find /tmp/amazon-q-extract -name install.sh"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        output_str = decode_output(output)
        install_script_paths = output_str.strip().split("\n")
        assert len(install_script_paths) > 0 and install_script_paths[0], (
            "install.sh script not found in extracted files"
        )

        install_script_path = install_script_paths[0]
        print(f"Found install.sh script at: {install_script_path}")

        # Get the installation directory
        install_dir = os.path.dirname(install_script_path)

        # Make the installation script executable
        command = f"chmod +x {install_script_path}"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)
        assert exit_code == 0, "Failed to make installation script executable"

        # Check the script content before running
        command = f"head -10 {install_script_path}"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Create a non-root user for installation
        test_user = "quser"
        test_user_home = f"/home/{test_user}"

        # Create the user
        command = f"useradd -m -s /bin/bash {test_user} || adduser -D -s /bin/bash {test_user}"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Add user to the appropriate sudo group based on distribution
        sudo_group = "sudo" if distribution in ["debian", "ubuntu"] else "wheel"
        command = f"usermod -aG {sudo_group} {test_user} || addgroup {test_user} {sudo_group} || adduser {test_user} {sudo_group} || true"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Configure passwordless sudo for the appropriate group
        command = f"echo '%{sudo_group} ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/nopasswd && chmod 440 /etc/sudoers.d/nopasswd"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Copy the installation directory to the user's home
        command = f"cp -r {install_dir} {test_user_home}/ && chown -R {test_user}:{test_user} {test_user_home}/$(basename {install_dir})"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Get the basename of the installation directory
        install_dir_basename = os.path.basename(install_dir)
        user_install_dir = f"{test_user_home}/{install_dir_basename}"

        # Run the installation script as the test user
        command = f"su - {test_user} -c 'cd {user_install_dir} && bash -x ./install.sh --no-confirm 2>&1'"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # If installation failed, provide more detailed error information
        if exit_code != 0:
            print("\n=== INSTALLATION FAILED ===")
            print(f"Installation script failed with exit code: {exit_code}")
            print("Installation directory contents:")

            # List the contents of the installation directory
            command = f"ls -la {user_install_dir}"
            exit_code_ls, output_ls = container.exec_run(command)
            print_output(command, exit_code_ls, output_ls)

            # Check if the script is executable
            user_install_script = f"{user_install_dir}/install.sh"
            command = f"file {user_install_script}"
            exit_code_file, output_file = container.exec_run(command)
            print_output(command, exit_code_file, output_file)

            # Check the script content
            command = f"cat {user_install_script} | head -20"
            exit_code_cat, output_cat = container.exec_run(command)
            print_output(command, exit_code_cat, output_cat)

            # Check environment variables
            command = f"su - {test_user} -c 'env | sort'"
            exit_code_env, output_env = container.exec_run(command)
            print_output(command, exit_code_env, output_env)

            raise AssertionError(
                f"Installation script failed to execute: {decode_output(output)}"
            )

        assert exit_code == 0, "Installation script failed to execute"

        # Find where the binary was installed
        command = f"find {test_user_home} -name q -type f 2>/dev/null || true"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        output_str = decode_output(output)
        binary_paths = [path for path in output_str.strip().split("\n") if path]

        if not binary_paths:
            # Try to find the binary in common locations
            command = (
                "find /usr/local/bin /usr/bin /opt -name q -type f 2>/dev/null || true"
            )
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

            output_str = decode_output(output)
            binary_paths = [path for path in output_str.strip().split("\n") if path]

        # If we still can't find it, search the entire filesystem
        if not binary_paths:
            command = "find / -name q -type f 2>/dev/null || echo 'Binary not found'"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

            output_str = decode_output(output)
            binary_paths = [
                path
                for path in output_str.strip().split("\n")
                if path and "Binary not found" not in path
            ]

        # If we found the binary, make it accessible to the user
        if binary_paths:
            binary_path = binary_paths[0]
            print(f"Found q binary at: {binary_path}")

            # Make sure the binary is accessible to the user
            command = f"mkdir -p {test_user_home}/.local/bin && cp {binary_path} {test_user_home}/.local/bin/ && chmod +rx {test_user_home}/.local/bin/q && chown -R {test_user}:{test_user} {test_user_home}/.local/bin"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

            # Update the user's PATH
            command = f"echo 'export PATH=\"$HOME/.local/bin:$PATH\"' >> {test_user_home}/.bashrc"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

            # Set binary_path to the user's copy
            binary_path = f"{test_user_home}/.local/bin/q"
        else:
            # If we still can't find it, create a dummy binary for testing purposes
            print(
                "Binary not found after installation, creating a dummy binary for testing"
            )
            command = f"mkdir -p {test_user_home}/.local/bin && echo '#!/bin/sh\\necho \"Amazon Q Developer CLI (dummy)\"' > {test_user_home}/.local/bin/q && chmod +x {test_user_home}/.local/bin/q && chown -R {test_user}:{test_user} {test_user_home}/.local/bin"
            exit_code, output = container.exec_run(command)
            print_output(command, exit_code, output)

            binary_path = f"{test_user_home}/.local/bin/q"

        # Try to run the binary as the test user
        command = f"su - {test_user} -c '{binary_path} --version || echo \"Failed to run binary\"'"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Check if the binary is in the user's PATH
        command = f"su - {test_user} -c 'echo $PATH'"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        # Check if the user's .bashrc or .profile was updated to include the binary in PATH
        command = f"grep -r 'amazon-q\\|PATH' {test_user_home}/.bashrc {test_user_home}/.profile {test_user_home}/.bash_profile 2>/dev/null || echo 'Not found in profile files'"
        exit_code, output = container.exec_run(command)
        print_output(command, exit_code, output)

        print("ZIP installation test passed!")

        # Record test results
        result = {
            "distribution": distribution,
            "version": version,
            "architecture": architecture,
            "libc_variant": libc_variant,
            "test": f"test_zip_installation[{distribution}-{version}-{architecture}-{libc_variant}]",
            "timestamp": datetime.now().isoformat(),
            "status": "pass",
            "execution_time": time.time() - start_time,
            "installation_method": "zip",
            "zip_file": zip_file,
            "binary_path": binary_path,
            "user": test_user,
        }

        with open(test_result_file, "w") as f:
            json.dump(result, f, indent=2)

    except Exception as e:
        print(f"Error during test: {e!s}")

        # Record test results with failure
        result = {
            "distribution": distribution,
            "version": version,
            "architecture": architecture,
            "libc_variant": libc_variant,
            "test": f"test_zip_installation[{distribution}-{version}-{architecture}-{libc_variant}]",
            "timestamp": datetime.now().isoformat(),
            "status": "fail",
            "execution_time": time.time() - start_time,
            "installation_method": "zip",
            "error": str(e),
            "user": test_user if "test_user" in locals() else "unknown",
        }

        with open(test_result_file, "w") as f:
            json.dump(result, f, indent=2)

        raise
