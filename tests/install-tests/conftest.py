#!/usr/bin/env python3
import os
import shutil
import subprocess
from pathlib import Path

import pytest


def detect_container_runtime():
    """Detect available container runtime."""
    if shutil.which("docker"):
        return "docker"
    elif shutil.which("finch"):
        return "finch"
    elif shutil.which("podman"):
        return "podman"
    else:
        raise RuntimeError(
            "No container runtime found. Please install Docker, Finch, or Podman."
        )


class ContainerWrapper:
    """Wrapper around container to provide a consistent interface."""

    def __init__(self, container_id, runtime="docker"):
        self.container_id = container_id
        self.runtime = runtime

    def exec_run(self, command):
        """Execute a command in the container and return the exit code and output."""
        runtime_cmd = [self.runtime, "exec", self.container_id, "sh", "-c", command]
        print(f"Executing in container: {' '.join(runtime_cmd)}")
        result = subprocess.run(runtime_cmd, capture_output=True, check=False)
        return result.returncode, result.stdout


@pytest.fixture(scope="function")
def container(request, distribution, version, architecture):
    """Create a container for the specified distribution and architecture."""
    runtime = request.config.getoption("--runtime") or detect_container_runtime()
    print(f"Using container runtime: {runtime}")

    # Map distribution and version to container image
    if distribution == "ubuntu":
        if version == "lts":
            # Use Ubuntu 22.04 LTS for x86_64 and arm64
            image = "ubuntu:22.04"
        else:
            image = f"ubuntu:{version}"
    elif distribution == "debian":
        image = f"debian:{version}"
    elif distribution == "fedora":
        image = f"fedora:{version}"
    elif distribution == "amazonlinux":
        image = f"amazonlinux:{version}"
    elif distribution == "alpine":
        image = f"alpine:{version}"
    elif distribution == "arch":
        image = "archlinux:latest"
    else:
        raise ValueError(f"Unsupported distribution: {distribution}")

    # Set platform flag for specific architectures
    platform_args = []
    if architecture == "arm64" or architecture == "aarch64":
        platform_args = ["--platform", f"linux/{architecture}"]
    elif architecture == "x86_64":
        platform_args = ["--platform", "linux/amd64"]

    # Pull the image with appropriate architecture
    print(f"Pulling image: {image} for architecture: {architecture}")
    pull_cmd = [runtime, "pull"]
    if platform_args:
        pull_cmd.extend(platform_args)
    pull_cmd.append(image)

    print(f"Running pull command: {' '.join(pull_cmd)}")
    subprocess.run(pull_cmd, check=True)

    # Create the container with appropriate architecture
    current_dir = os.path.abspath(".")
    run_cmd = [runtime, "run", "-d"]

    # Add platform flag for specific architectures
    if platform_args:
        run_cmd.extend(platform_args)

    # Add volume mount and other parameters
    run_cmd.extend(
        ["-v", f"{current_dir}:/amazon-q-developer-cli:ro", image, "sleep", "3600"]
    )

    print(f"Creating container with command: {' '.join(run_cmd)}")
    container_id = subprocess.check_output(run_cmd).decode().strip()

    print(f"Created container: {container_id}")

    # Wrap the container
    wrapper = ContainerWrapper(container_id, runtime)

    # Yield the container wrapper
    yield wrapper

    # Stop and remove the container
    print(f"Stopping container: {container_id}")
    subprocess.run([runtime, "stop", container_id], check=True)
    print(f"Removing container: {container_id}")
    subprocess.run([runtime, "rm", container_id], check=True)


@pytest.fixture(scope="function")
def package_path(distribution, architecture):
    """Return the path to the package for the specified distribution and architecture."""
    if distribution in ["debian", "ubuntu"]:
        if architecture == "x86_64":
            return "/amazon-q-developer-cli/bundle/deb/amazon-q-developer-cli_amd64.deb"
        elif architecture == "arm64":
            return "/amazon-q-developer-cli/bundle/deb/amazon-q-developer-cli_arm64.deb"
    elif distribution in ["fedora", "amazonlinux"]:
        if architecture == "x86_64":
            return (
                "/amazon-q-developer-cli/bundle/rpm/amazon-q-developer-cli.x86_64.rpm"
            )
        elif architecture == "arm64":
            return (
                "/amazon-q-developer-cli/bundle/rpm/amazon-q-developer-cli.aarch64.rpm"
            )
    elif distribution == "alpine":
        if architecture == "x86_64":
            return "/amazon-q-developer-cli/bundle/apk/amazon-q-developer-cli.apk"
    elif distribution == "arch":
        if architecture == "x86_64":
            return (
                "/amazon-q-developer-cli/bundle/pkg/amazon-q-developer-cli.pkg.tar.zst"
            )

    raise ValueError(
        f"Unsupported distribution/architecture combination: {distribution}/{architecture}"
    )


@pytest.fixture(scope="function")
def test_result_file(distribution, version, architecture, request):
    """Return the path to the test result file."""
    results_dir = Path("results")
    results_dir.mkdir(exist_ok=True)

    test_name = request.node.name.split("[")[0]
    return results_dir / f"{distribution}-{version}-{architecture}-{test_name}.json"


@pytest.fixture(scope="function")
def is_known_failure(distribution, version, architecture):
    """Check if the test is expected to fail for the specified distribution and architecture."""
    known_failures = {
        # Format: ("distribution", "version", "architecture"): "reason"
        ("alpine", "latest", "x86_64"): "Alpine package not yet supported",
        ("arch", "latest", "x86_64"): "Arch package not yet supported",
    }

    key = (distribution, version, architecture)
    if key in known_failures:
        return (True, known_failures[key])

    return (False, None)


def pytest_addoption(parser):
    """Add command line options for specifying distribution, version, and architecture."""
    parser.addoption(
        "--distribution", action="store", default="ubuntu", help="Distribution to test"
    )
    parser.addoption(
        "--dist-version",
        action="store",
        default="lts",
        help="Distribution version to test",
    )
    parser.addoption(
        "--architecture", action="store", default="x86_64", help="Architecture to test"
    )
    parser.addoption(
        "--runtime",
        action="store",
        help="Container runtime to use (docker, finch, podman)",
    )
    parser.addoption(
        "--libc-variant",
        action="store",
        default="glibc",
        choices=["musl", "glibc"],
        help="C library variant to use (musl or glibc)",
    )


@pytest.fixture(scope="function")
def distribution(request):
    """Return the distribution specified on the command line."""
    return request.config.getoption("--distribution")


@pytest.fixture(scope="function")
def version(request):
    """Return the distribution version specified on the command line."""
    return request.config.getoption("--dist-version")


@pytest.fixture(scope="function")
def architecture(request):
    """Return the architecture specified on the command line."""
    return request.config.getoption("--architecture")


@pytest.fixture(scope="function")
def runtime(request):
    """Return the container runtime specified on the command line or detect it."""
    runtime = request.config.getoption("--runtime")
    if not runtime:
        runtime = detect_container_runtime()
    return runtime


@pytest.fixture(scope="function")
def libc_variant(request):
    """Return the C library variant specified on the command line."""
    return request.config.getoption("--libc-variant")
