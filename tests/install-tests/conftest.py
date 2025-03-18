#!/usr/bin/env python3
import json
import os
import shutil
import subprocess
from datetime import datetime
from pathlib import Path

import pytest


def detect_container_runtime():
    """Detect available container runtime, prioritizing Finch over Podman over Docker."""
    if shutil.which("finch"):
        return "finch"
    elif shutil.which("podman"):
        return "podman"
    elif shutil.which("docker"):
        return "docker"
    else:
        raise RuntimeError(
            "No container runtime found. Please install Docker, Finch, or Podman."
        )


class ContainerWrapper:
    """Wrapper around container to provide a consistent interface."""

    def __init__(self, container_id, runtime="docker"):
        self.container_id = container_id
        self.runtime = runtime

    def exec_run(self, command, timeout=None):
        """Execute a command in the container and return the exit code and output."""
        runtime_cmd = [self.runtime, "exec", self.container_id, "sh", "-c", command]
        print(f"Executing in container: {' '.join(runtime_cmd)}")
        # Don't use timeout parameter for now
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
    elif distribution == "rocky":
        image = f"rockylinux:{version}"
    elif distribution == "centos":
        if version.startswith("stream"):
            stream_version = version.replace("stream", "")
            image = f"quay.io/centos/centos:stream{stream_version}"
        else:
            image = f"centos:{version}"
    elif distribution == "opensuse":
        image = f"opensuse/leap:{version}"
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
def install_dir(request, libc_variant, architecture):
    """Return the path to the installation directory."""
    # Use the appropriate zip file based on architecture and libc variant
    zip_dir = "/amazon-q-developer-cli/bundle/zip"

    arch_name = "aarch64" if architecture == "arm64" else architecture

    if libc_variant == "musl":
        return f"{zip_dir}/amazon-q-developer-cli-{arch_name}-linux-musl.zip"
    else:
        return f"{zip_dir}/amazon-q-developer-cli-{arch_name}-linux.zip"


# Define test matrix with glibc version information
# Format: (distribution, version, architecture, min_glibc_version)
DISTROS = [
    # Ubuntu
    ("ubuntu", "20.04", "x86_64", "2.31"),  # Ubuntu 20.04 has glibc 2.31
    ("ubuntu", "20.04", "arm64", "2.31"),
    ("ubuntu", "22.04", "x86_64", "2.35"),  # Ubuntu 22.04 has glibc 2.35
    ("ubuntu", "22.04", "arm64", "2.35"),
    ("ubuntu", "24.04", "x86_64", "2.38"),  # Ubuntu 24.04 has glibc 2.38
    ("ubuntu", "24.04", "arm64", "2.38"),
    # Debian
    ("debian", "11", "x86_64", "2.31"),  # Debian 11 has glibc 2.31
    ("debian", "11", "arm64", "2.31"),
    ("debian", "12", "x86_64", "2.36"),  # Debian 12 has glibc 2.36
    ("debian", "12", "arm64", "2.36"),
    # Fedora
    ("fedora", "38", "x86_64", "2.37"),  # Fedora 38 has glibc 2.37
    ("fedora", "39", "x86_64", "2.38"),  # Fedora 39 has glibc 2.38
    # Amazon Linux
    ("amazonlinux", "2023", "x86_64", "2.34"),  # Amazon Linux 2023 has glibc 2.34
    ("amazonlinux", "2023", "arm64", "2.34"),
    ("amazonlinux", "2", "x86_64", "2.26"),  # Amazon Linux 2 has glibc 2.26
    # Rocky Linux
    ("rocky", "9", "x86_64", "2.34"),  # Rocky Linux 9 has glibc 2.34
    ("rocky", "9", "arm64", "2.34"),
    # Alpine (musl-based)
    ("alpine", "3.19", "x86_64", None),  # Alpine uses musl, not glibc
    ("alpine", "3.19", "arm64", None),
]

# Minimum glibc version required for our glibc binary
REQUIRED_GLIBC_VERSION = "2.34"


def parse_version(version_str):
    """Parse version string into tuple for comparison."""
    if not version_str:
        return (0, 0)
    parts = version_str.split(".")
    return (int(parts[0]), int(parts[1]) if len(parts) > 1 else 0)


# Generate test parameters
TEST_PARAMS = []
for distro, version, arch, glibc_version in DISTROS:
    # All distributions can use musl
    TEST_PARAMS.append((distro, version, arch, "musl"))

    # Only distributions with sufficient glibc version can use glibc
    if glibc_version and parse_version(glibc_version) >= parse_version(
        REQUIRED_GLIBC_VERSION
    ):
        TEST_PARAMS.append((distro, version, arch, "glibc"))


def pytest_addoption(parser):
    """Add command line options for specifying distribution, version, and architecture."""
    parser.addoption("--distribution", action="store", help="Distribution to test")
    parser.addoption(
        "--dist-version",
        action="store",
        help="Distribution version to test",
    )
    parser.addoption("--architecture", action="store", help="Architecture to test")
    parser.addoption(
        "--runtime",
        action="store",
        help="Container runtime to use (docker, finch, podman)",
    )
    parser.addoption(
        "--libc-variant",
        action="store",
        choices=["musl", "glibc"],
        help="C library variant to use (musl or glibc)",
    )


def pytest_generate_tests(metafunc):
    """Generate test combinations based on the test matrix."""
    if all(
        param in metafunc.fixturenames
        for param in ["distribution", "version", "architecture", "libc_variant"]
    ):
        # If specific parameters were provided via command line, use those
        if (
            metafunc.config.getoption("--distribution")
            and metafunc.config.getoption("--dist-version")
            and metafunc.config.getoption("--architecture")
            and metafunc.config.getoption("--libc-variant")
        ):
            params = [
                (
                    metafunc.config.getoption("--distribution"),
                    metafunc.config.getoption("--dist-version"),
                    metafunc.config.getoption("--architecture"),
                    metafunc.config.getoption("--libc-variant"),
                )
            ]
        else:
            # Otherwise use the full test matrix
            params = TEST_PARAMS

        metafunc.parametrize("distribution,version,architecture,libc_variant", params)


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
def libc_variant(request):
    """Return the C library variant specified on the command line."""
    return request.config.getoption("--libc-variant")


def pytest_sessionfinish(session, exitstatus):
    """Generate a summary report after all tests have completed."""
    results_dir = Path("results")
    if not results_dir.exists():
        return

    results = []
    for result_file in results_dir.glob("*.json"):
        if result_file.name == "zip_test_summary.json":
            continue
        try:
            with open(result_file) as f:
                results.append(json.load(f))
        except json.JSONDecodeError:
            print(f"Warning: Could not parse JSON file: {result_file}")
            continue

    if not results:
        return

    total = len(results)
    passed = sum(1 for r in results if r.get("status") == "pass")
    failed = sum(1 for r in results if r.get("status") == "fail")
    skipped = sum(1 for r in results if r.get("status") == "skip")

    summary = {
        "timestamp": datetime.now().isoformat(),
        "total_tests": total,
        "passed": passed,
        "failed": failed,
        "skipped": skipped,
        "success_rate": passed / (total - skipped) * 100 if total > skipped else 0,
        "results_by_distribution": {},
        "results_by_architecture": {},
        "results_by_libc_variant": {},
        "failed_tests": [],
        "skipped_tests": [],
        "detailed_results": results,
    }

    # Group results by distribution
    for result in results:
        dist_key = f"{result['distribution']} {result['version']}"
        if dist_key not in summary["results_by_distribution"]:
            summary["results_by_distribution"][dist_key] = {
                "total": 0,
                "passed": 0,
                "failed": 0,
                "skipped": 0,
            }

        summary["results_by_distribution"][dist_key]["total"] += 1
        if result["status"] == "pass":
            summary["results_by_distribution"][dist_key]["passed"] += 1
        elif result["status"] == "fail":
            summary["results_by_distribution"][dist_key]["failed"] += 1
            summary["failed_tests"].append(result)
        else:
            summary["results_by_distribution"][dist_key]["skipped"] += 1
            summary["skipped_tests"].append(result)

    # Write summary to file
    summary_file = results_dir / "zip_test_summary.json"
    with open(summary_file, "w") as f:
        json.dump(summary, f, indent=2)

    # Print summary to console
    print("\n" + "=" * 80)
    print("ZIP INSTALLATION TEST SUMMARY")
    print("=" * 80)
    print(f"Total tests: {total}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Skipped: {skipped}")
    print(
        f"Success rate: {passed / (total - skipped) * 100:.1f}% (excluding skipped tests)"
    )

    print("\nResults by distribution:")
    for dist, counts in summary["results_by_distribution"].items():
        total_run = counts["total"] - counts["skipped"]
        if total_run > 0:
            success_rate = counts["passed"] / total_run * 100
        else:
            success_rate = 0
        print(
            f"  {dist}: {counts['passed']}/{total_run} passed ({success_rate:.1f}%), {counts['skipped']} skipped"
        )

    if failed:
        print("\nFailed tests:")
        for test in summary["failed_tests"]:
            print(
                f"  {test['distribution']} {test['version']} on {test['architecture']} with {test['libc_variant']}"
            )

    if skipped:
        print("\nSkipped tests:")
        for test in summary["skipped_tests"]:
            print(
                f"  {test['distribution']} {test['version']} on {test['architecture']} with {test['libc_variant']}"
            )

    print(f"\nDetailed summary written to {summary_file}")
