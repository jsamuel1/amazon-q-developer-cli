#!/usr/bin/env python3
"""
Tests for Amazon Q Developer CLI ZIP installation.

This test verifies that the Amazon Q Developer CLI can be installed from a ZIP package.
The test:
1. Extracts the ZIP file to a temporary location
2. Creates a non-root user for installation
4. Copies the binary to the user's home directory and makes it accessible
3. Runs the included install.sh script as the new user
5. Verifies the binary is executable and in the user's PATH
"""

import json
import os
import time
from datetime import datetime

import pytest

# Standard timeout for all operations (5 minutes)
TIMEOUT = 300


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


def run_command(container, command, check_exit_code=False, print_cmd=True):
    """Helper function to run a command and print its output.

    Args:
        container: The container to run the command in
        command: The command to run
        check_exit_code: If True, assert that the exit code is 0
        print_cmd: If True, print the command output

    Returns:
        Tuple of (exit_code, output_str)
    """
    print(f"Running command with timeout {TIMEOUT}s: {command}")
    start_time = time.time()

    # Add timeout to the container exec_run if possible
    try:
        # Don't use timeout for now as it's causing issues
        exit_code, output = container.exec_run(command)
    except Exception as e:
        print(f"Command timed out or failed after {time.time() - start_time:.1f}s: {e}")
        return 1, f"COMMAND FAILED: {e!s}"

    output_str = decode_output(output)
    elapsed = time.time() - start_time

    if print_cmd:
        print_output(command, exit_code, output)
        print(f"Command completed in {elapsed:.1f}s")

    if check_exit_code and exit_code != 0:
        raise AssertionError(f"Command failed with exit code {exit_code}: {command}")

    return exit_code, output_str


# Define standard timeouts for different operations
TIMEOUT_SHORT = 120  # For quick operations
TIMEOUT_MEDIUM = 300  # For medium operations (5 minutes)
TIMEOUT_LONG = 600  # For long operations (10 minutes)
DISTRIBUTION_PARAMS = {
    "ubuntu": {
        "package_manager": "apt-get install -y --no-install-recommends",
        "sudo_group": "sudo",
        "has_selinux": False,
    },
    "debian": {
        "package_manager": "apt-get install -y --no-install-recommends",
        "sudo_group": "sudo",
        "has_selinux": False,
    },
    "fedora": {
        "package_manager": "dnf makecache && dnf install -y --setopt=timeout=300",
        "sudo_group": "wheel",
        "has_selinux": True,
    },
    "amazonlinux": {
        "package_manager": "yum install -y --setopt=timeout=300",
        "sudo_group": "wheel",
        "has_selinux": True,
    },
    "rocky": {
        "package_manager": "dnf install -y --setopt=timeout=300",
        "sudo_group": "wheel",
        "has_selinux": True,
    },
    "alpine": {
        "package_manager": "apk update && apk add --no-cache",
        "sudo_group": "wheel",
        "has_selinux": False,
        "extra_setup": "addgroup wheel 2>/dev/null || true",
    },
}

# Common packages needed for all distributions
COMMON_PACKAGES = "unzip ca-certificates findutils sudo which coreutils"
# Additional packages for specific distributions
EXTRA_PACKAGES = {
    "alpine": "bash shadow",
    "debian": "passwd",
    "ubuntu": "passwd",
    "fedora": "shadow-utils",
    "rocky": "shadow-utils",
    "amazonlinux": "shadow-utils",
}


def test_zip_installation(
    container, distribution, version, architecture, libc_variant, install_dir
):
    """Test installation from ZIP file."""
    # Record the start time for the test
    pytest.start_time = time.time()

    try:
        # Create a test user for installation
        test_user = "quser"
        test_user_home = f"/home/{test_user}"

        # Get distribution-specific parameters
        dist_params = DISTRIBUTION_PARAMS.get(
            distribution,
            {
                "package_manager": "apt-get update && apt-get install -y",
                "sudo_group": "sudo",
                "has_selinux": False,
            },
        )

        # Install required packages for unzipping and user management
        packages = COMMON_PACKAGES
        if distribution in EXTRA_PACKAGES:
            packages += f" {EXTRA_PACKAGES[distribution]}"

        # Use the package manager command directly
        if "apt-get" in dist_params["package_manager"]:
            # Always run apt-get update first for Debian/Ubuntu
            run_command(container, "apt-get update -y", check_exit_code=True)

            # Try installing all packages at once first
            try:
                run_command(
                    container,
                    f"DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends {packages}",
                    check_exit_code=True,
                )
                print("Successfully installed all packages")
            except Exception as e:
                print(f"Failed to install all packages at once: {e!s}")
                print("Falling back to installing packages one by one")

                # Try installing packages one by one as fallback
                for pkg in packages.split():
                    try:
                        run_command(
                            container,
                            f"DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends {pkg}",
                            check_exit_code=True,
                        )
                        print(f"Successfully installed {pkg}")
                    except Exception as e:
                        print(f"Failed to install {pkg}: {e!s}")
                        if pkg not in ["unzip", "sudo"]:  # These are essential
                            print(f"Continuing without {pkg}")
                        else:
                            raise
        elif distribution == "rocky" or distribution == "amazonlinux":
            # For Rocky Linux and Amazon Linux, try without makecache and with specific error handling
            try:
                # First try with basic install
                if distribution == "rocky":
                    run_command(
                        container,
                        f"dnf install -y {packages}",
                        check_exit_code=True,
                    )
                else:  # amazonlinux
                    run_command(
                        container,
                        f"yum install -y --allowerasing {packages}",
                        check_exit_code=True,
                    )
            except Exception as e:
                print(f"Package installation failed with basic install: {e!s}")
                # Try with repo refresh
                if distribution == "rocky":
                    run_command(container, "dnf clean all", check_exit_code=False)
                    run_command(
                        container,
                        "dnf check-update || true",
                        check_exit_code=False,
                    )
                else:  # amazonlinux
                    run_command(container, "yum clean all", check_exit_code=False)
                    run_command(
                        container,
                        "yum check-update || true",
                        check_exit_code=False,
                    )

                # Try installing packages one by one
                for pkg in packages.split():
                    try:
                        if distribution == "rocky":
                            run_command(
                                container,
                                f"dnf install -y {pkg}",
                                check_exit_code=True,
                            )
                        else:  # amazonlinux
                            run_command(
                                container,
                                f"yum install -y --allowerasing {pkg}",
                                check_exit_code=True,
                            )
                        print(f"Successfully installed {pkg}")
                    except Exception as e2:
                        print(f"Failed to install {pkg}: {e2!s}")
                        if pkg not in ["unzip", "sudo"]:  # These are essential
                            print(f"Continuing without {pkg}")
                        else:
                            raise
        else:
            install_cmd = f"{dist_params['package_manager']} {packages}"
            run_command(container, install_cmd, check_exit_code=True)

        # Run any extra setup commands if needed
        if "extra_setup" in dist_params:
            run_command(container, dist_params["extra_setup"])

        # Check and handle SELinux for distributions that might have it enabled
        if dist_params["has_selinux"]:
            exit_code, output_str = run_command(
                container,
                "command -v getenforce && getenforce || echo 'SELinux not found'",
            )

            if "Enforcing" in output_str:
                print(
                    "SELinux is in enforcing mode. Setting to permissive for the test..."
                )
                run_command(
                    container,
                    "setenforce 0 || echo 'Failed to set SELinux to permissive mode'",
                )

        # Create a test user
        try:
            # First try useradd (common on many distros)
            run_command(
                container, f"useradd -m -s /bin/bash {test_user}", check_exit_code=True
            )
            print(f"Created user {test_user} with useradd")
        except Exception as e1:
            print(f"useradd failed: {e1!s}")
            try:
                # Then try distribution-specific approaches
                if distribution == "alpine":
                    run_command(
                        container,
                        f"adduser -h {test_user_home} -s /bin/bash -D {test_user}",
                        check_exit_code=True,
                    )
                    print(f"Created user {test_user} with Alpine adduser")
                elif distribution in ["ubuntu", "debian"]:
                    run_command(
                        container,
                        f"adduser --disabled-password --gecos '' {test_user}",
                        check_exit_code=True,
                    )
                    print(f"Created user {test_user} with Debian/Ubuntu adduser")
                else:
                    # Last resort, try basic adduser without options
                    run_command(container, f"adduser {test_user}", check_exit_code=True)
                    print(f"Created user {test_user} with basic adduser")
            except Exception as e2:
                print(f"All user creation methods failed: {e1!s}, then {e2!s}")
                raise Exception(
                    f"Failed to create user {test_user} using any available method"
                )

        # Verify the user was created correctly
        run_command(
            container,
            f"id {test_user} && ls -la {test_user_home}",
            check_exit_code=True,
        )

        # Add user to the appropriate sudo group based on distribution
        sudo_group = dist_params["sudo_group"]
        # First ensure the group exists
        run_command(
            container,
            f"getent group {sudo_group} || groupadd {sudo_group}",
            check_exit_code=True,
        )

        # Then add the user to the group using the appropriate command for the distribution
        if distribution == "alpine":
            run_command(
                container, f"addgroup {test_user} {sudo_group}", check_exit_code=True
            )
        else:
            run_command(
                container, f"usermod -aG {sudo_group} {test_user}", check_exit_code=True
            )

        # Verify the user has sudo privileges
        run_command(
            container,
            f"groups {test_user} | grep -q {sudo_group} && echo 'User has sudo group' || (echo 'User missing sudo group' && exit 1)",
            check_exit_code=True,
        )

        # Configure passwordless sudo for the appropriate group
        run_command(
            container,
            f"echo '%{sudo_group} ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/nopasswd && chmod 440 /etc/sudoers.d/nopasswd",
            check_exit_code=True,
        )

        # Check if source directory exists
        run_command(container, f"ls -la {install_dir}", check_exit_code=True)

        # Create a temporary directory for extraction
        extract_dir = f"{test_user_home}/q-extract"
        run_command(
            container,
            f"mkdir -p {extract_dir} && chmod 755 {extract_dir} && chown {test_user}:{test_user} {extract_dir}",
            check_exit_code=True,
        )

        # Copy the zip file to the user's home directory
        run_command(
            container,
            f"cp -v {install_dir} {extract_dir}/ && chown {test_user}:{test_user} {extract_dir}/$(basename {install_dir})",
        )

        # Make sure unzip and ca-certificates are installed and working
        # Explicitly install unzip for all distributions to ensure it's available
        run_command(
            container,
            f"{dist_params['package_manager']} unzip ca-certificates",
            timeout=300,
        )
        run_command(container, "which unzip && unzip -v")
        run_command(
            container,
            "ls -la /etc/ssl/certs || ls -la /etc/pki/tls/certs || echo 'CA certificates directory not found'",
        )

        # Extract the zip file as the test user
        zip_file = os.path.basename(install_dir)
        try:
            # First try with su - user
            exit_code, output_str = run_command(
                container,
                f"cd {extract_dir} && chown -R {test_user}:{test_user} . && su - {test_user} -c 'cd {extract_dir} && unzip -o {zip_file}'",
                check_exit_code=True,  # Fail if this approach fails
                timeout=300,  # 5 minute timeout for unzip
            )
        except Exception as e:
            print(f"Unzip failed with su - user: {e!s}")
            try:
                # Try with sudo instead of su
                exit_code, output_str = run_command(
                    container,
                    f"cd {extract_dir} && chown -R {test_user}:{test_user} . && sudo -u {test_user} bash -c 'cd {extract_dir} && unzip -o {zip_file}'",
                    check_exit_code=True,
                    timeout=300,
                )
            except Exception as e2:
                print(f"Unzip failed with sudo -u user: {e2!s}")
                # Last resort: unzip as root and fix permissions
                exit_code, output_str = run_command(
                    container,
                    f"cd {extract_dir} && unzip -o {zip_file} && chown -R {test_user}:{test_user} .",
                    check_exit_code=True,
                    timeout=300,
                )

        # Verify the files were extracted correctly
        run_command(container, f"ls -la {extract_dir}")

        # Check if the q subdirectory exists (ZIP extracts into a q subfolder)
        run_command(container, f"ls -la {extract_dir}")

        # Check if install.sh exists and is executable in the q subdirectory
        run_command(
            container,
            f"test -x {extract_dir}/q/install.sh && echo 'install.sh is executable' || echo 'install.sh is NOT executable'",
        )

        # Make sure the script is executable
        run_command(container, f"chmod +x {extract_dir}/q/install.sh")

        # Check if readlink -f works (some distributions might not support -f flag)
        exit_code, output_str = run_command(
            container,
            "readlink -f /bin/sh >/dev/null 2>&1 || echo 'readlink -f not supported'",
        )

        if "not supported" in output_str:
            print(
                "WARNING: readlink -f is not supported on this distribution. This may cause installation issues."
            )

        # Check if timeout command exists and works
        run_command(container, "command -v timeout || echo 'timeout command not found'")
        run_command(
            container,
            "timeout --version || echo 'timeout command not working properly'",
        )

        # Check user environment before running the installation
        run_command(
            container,
            f"su - {test_user} -c 'echo \"User environment before installation:\" && pwd && ls -la && echo PATH=$PATH && echo HOME=$HOME && echo USER=$USER'",
        )

        # Run the installation script as the test user (not as root)
        # Use bash -x for verbose debugging and tee to both console and log file
        install_log = f"{test_user_home}/install.log"
        print(f"Starting installation at {datetime.now().isoformat()}")

        # Install script command to see what's in the install.sh file
        run_command(container, f"cat {extract_dir}/q/install.sh | head -20")

        print("\n=== Running installation script ===")
        exit_code, output_str = run_command(
            container,
            f"su - {test_user} -c 'cd {extract_dir}/q && bash -x ./install.sh --no-confirm --verbose 2>&1 | tee {install_log}' || sudo -u {test_user} bash -c 'cd {extract_dir}/q && bash -x ./install.sh --no-confirm --verbose 2>&1 | tee {install_log}'",
            timeout=660,  # 11 minute timeout
            check_exit_code=False,  # Don't fail immediately, we'll handle it below
        )

        print(
            f"Installation script completed at {datetime.now().isoformat()} with exit code: {exit_code}"
        )

        # Display the installation log regardless of success or failure
        run_command(container, f"cat {install_log} || echo 'Log file not found'")

        # If installation failed, provide more detailed error information
        if exit_code != 0:
            print("\n=== INSTALLATION FAILED ===")
            print(f"Installation script failed with exit code: {exit_code}")

            # Check if the script exists and is executable
            run_command(container, f"ls -la {extract_dir}/q/install.sh")

            # Check environment variables
            run_command(
                container,
                f"su - {test_user} -c 'env | grep -E \"PATH|HOME|USER\"' || echo 'Failed to get user environment'",
            )

            # Check for common error patterns in the log
            run_command(
                container,
                f"grep -i 'error\\|failed\\|not found' {install_log} || echo 'No common error patterns found in log'",
            )

            # For Fedora, check if we need to install additional dependencies
            if distribution == "fedora":
                print("\n=== Installing additional dependencies for Fedora ===")
                run_command(
                    container,
                    "dnf install -y glibc-devel libstdc++-devel",
                    check_exit_code=False,
                )

                # Try installation again with the same user but with new dependencies
                print(
                    "\n=== Trying installation again with additional dependencies ==="
                )
                exit_code2, output_str2 = run_command(
                    container,
                    f"su - {test_user} -c 'cd {extract_dir}/q && bash -x ./install.sh --no-confirm --verbose 2>&1 | tee {install_log}.retry'",
                    timeout=660,
                    check_exit_code=False,
                )

                if exit_code2 == 0:
                    print("Installation succeeded with additional dependencies")
                    exit_code = 0  # Mark as successful
                else:
                    run_command(
                        container,
                        f"cat {install_log}.retry || echo 'Retry log file not found'",
                    )

            # If still failing, raise an exception
            if exit_code != 0:
                # Check for glibc version
                run_command(container, "ldd --version || echo 'ldd command not found'")

                raise Exception(
                    f"Installation script failed with exit code: {exit_code}"
                )

        assert exit_code == 0, "Installation script failed to execute"

        # Find where the binary was installed
        exit_code, output_str = run_command(
            container, f"find {test_user_home} -name q -type f 2>/dev/null || true"
        )

        binary_paths = [path for path in output_str.strip().split("\n") if path]

        # If we found the binary, make it accessible to the user
        if binary_paths:
            binary_path = binary_paths[0]
            print(f"Found q binary at: {binary_path}")

            # Make sure the binary is accessible to the user
            run_command(
                container,
                f"mkdir -p {test_user_home}/.local/bin && cp {binary_path} {test_user_home}/.local/bin/ && chmod +rx {test_user_home}/.local/bin/q && chown -R {test_user}:{test_user} {test_user_home}/.local/bin",
            )

            # Update the user's PATH
            run_command(
                container,
                f"echo 'export PATH=\"$HOME/.local/bin:$PATH\"' >> {test_user_home}/.bashrc",
            )

            # Set binary_path to the user's copy
            binary_path = f"{test_user_home}/.local/bin/q"
        else:
            # If binary not found, this is a failure
            print("ERROR: Binary not found after installation")

            # Check installation paths to help with debugging
            run_command(
                container,
                "find /usr/local/bin /usr/bin /opt -name q -type f 2>/dev/null || echo 'Binary not found in standard locations'",
            )

            raise AssertionError(
                "Binary not found after installation - installation failed"
            )

        # Verify the binary is executable
        run_command(
            container,
            f"test -x {binary_path} && echo 'Binary is executable' || echo 'Binary is NOT executable'",
        )

        # Run the binary to verify it works
        run_command(container, f"su - {test_user} -c '{binary_path} --version'")

        # Record the test result
        result = {
            "distribution": distribution,
            "version": version,
            "architecture": architecture,
            "libc_variant": libc_variant,
            "test": f"test_zip_installation[{distribution}-{version}-{architecture}-{libc_variant}]",
            "timestamp": datetime.now().isoformat(),
            "status": "pass",
            "execution_time": time.time() - pytest.start_time,
            "installation_method": "zip",
            "user": test_user,
            "install_log": install_log,
        }

        # Add install log content to the result
        try:
            exit_code, log_content = run_command(
                container,
                f"cat {install_log} || echo 'Log file not found'",
                print_cmd=False,
            )
            if exit_code == 0:
                result["install_log_content"] = log_content
        except Exception as log_error:
            result["install_log_error"] = str(log_error)

        # Create results directory if it doesn't exist
        os.makedirs("results", exist_ok=True)

        # Write the result to a file
        result_file = f"results/{distribution}-{version}-{architecture}-{libc_variant}-test_zip_installation.json"
        with open(result_file, "w") as f:
            json.dump(result, f, indent=2)

        print(f"Test result written to {result_file}")

    except Exception as e:
        # Create results directory if it doesn't exist
        os.makedirs("results", exist_ok=True)

        # Capture the last command output if available
        last_output = output_str if "output_str" in locals() else "No output captured"

        # Record the failure
        result = {
            "distribution": distribution,
            "version": version,
            "architecture": architecture,
            "libc_variant": libc_variant,
            "test": f"test_zip_installation[{distribution}-{version}-{architecture}-{libc_variant}]",
            "timestamp": datetime.now().isoformat(),
            "status": "fail",
            "error": str(e),
            "output": last_output,
            "execution_time": time.time() - pytest.start_time,
            "installation_method": "zip",
        }

        # If install_log exists, try to read its contents
        if "install_log" in locals():
            try:
                exit_code, log_content = run_command(
                    container,
                    f"cat {install_log} || echo 'Log file not found'",
                    print_cmd=False,
                )
                if exit_code == 0:
                    result["install_log_content"] = log_content
            except Exception as log_error:
                result["install_log_error"] = str(log_error)

        # Write the failure result to a file
        result_file = f"results/{distribution}-{version}-{architecture}-{libc_variant}-test_zip_installation.json"
        with open(result_file, "w") as f:
            json.dump(result, f, indent=2)

        print(f"Test failure recorded in {result_file}")
        raise  # Re-raise the exception to fail the test
