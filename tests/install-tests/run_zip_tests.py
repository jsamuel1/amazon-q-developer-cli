#!/usr/bin/env python3
"""
Script to run all variants of the ZIP installation tests.

This script runs the ZIP installation test for various combinations of:
- Distributions
- Versions
- Architectures
- C library variants (musl and glibc)

Usage:
    python run_zip_tests.py [--runtime RUNTIME] [--distributions DIST1,DIST2,...] [--architectures ARCH1,ARCH2,...]
"""

import argparse
import json
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path

# Default test matrix
DEFAULT_DISTRIBUTIONS = [
    ("ubuntu", "22.04"),
    ("ubuntu", "20.04"),
    ("debian", "11"),
    ("debian", "12"),
    ("amazonlinux", "2"),
    ("amazonlinux", "2023"),
    ("fedora", "38"),
    ("alpine", "latest"),
]

DEFAULT_ARCHITECTURES = ["x86_64", "arm64"]
DEFAULT_LIBC_VARIANTS = ["musl", "glibc"]  # Removed "auto"


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Run ZIP installation tests for multiple distributions and architectures"
    )
    parser.add_argument(
        "--runtime", help="Container runtime to use (docker, finch, podman)"
    )
    parser.add_argument(
        "--distributions",
        help="Comma-separated list of distributions to test (e.g., ubuntu:22.04,debian:11)",
    )
    parser.add_argument(
        "--architectures",
        help="Comma-separated list of architectures to test (e.g., x86_64,arm64)",
    )
    parser.add_argument(
        "--libc-variants",
        help="Comma-separated list of libc variants to test (musl,glibc)",
    )
    parser.add_argument(
        "--no-summary",
        action="store_true",
        help="Don't generate a summary report after running tests",
    )
    parser.add_argument(
        "--parallel", action="store_true", help="Run tests in parallel (experimental)"
    )
    return parser.parse_args()


def run_test(distribution, version, architecture, libc_variant, runtime=None):
    """Run a single ZIP installation test."""
    print(f"\n{'=' * 80}")
    print(
        f"Testing {distribution} {version} on {architecture} with libc variant: {libc_variant}"
    )
    print(f"{'=' * 80}\n")

    cmd = [
        "python",
        "-m",
        "pytest",
        "tests/test_zip_installation.py::test_zip_installation",
        f"--distribution={distribution}",
        f"--dist-version={version}",
        f"--architecture={architecture}",
        f"--libc-variant={libc_variant}",
        "-v",
    ]

    if runtime:
        cmd.append(f"--runtime={runtime}")

    start_time = time.time()
    result = subprocess.run(cmd, capture_output=True, text=True, check=False)
    duration = time.time() - start_time

    # Print the output
    print(result.stdout)
    if result.stderr:
        print(result.stderr)

    success = result.returncode == 0
    status = "PASS" if success else "FAIL"

    print(
        f"\n{status}: {distribution} {version} on {architecture} with libc variant: {libc_variant}"
    )
    print(f"Duration: {duration:.2f} seconds")

    return {
        "distribution": distribution,
        "version": version,
        "architecture": architecture,
        "libc_variant": libc_variant,
        "status": status,
        "duration": duration,
        "timestamp": datetime.now().isoformat(),
        "returncode": result.returncode,
    }


def generate_summary(results):
    """Generate a summary of test results."""
    total = len(results)
    passed = sum(1 for r in results if r["status"] == "PASS")
    failed = total - passed

    print("\n" + "=" * 80)
    print("ZIP INSTALLATION TEST SUMMARY")
    print("=" * 80)
    print(f"Total tests: {total}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Success rate: {passed / total * 100:.1f}%")
    print("\nResults by distribution:")

    # Group by distribution and version
    by_dist = {}
    for r in results:
        key = f"{r['distribution']} {r['version']}"
        if key not in by_dist:
            by_dist[key] = {"total": 0, "passed": 0, "failed": 0}
        by_dist[key]["total"] += 1
        if r["status"] == "PASS":
            by_dist[key]["passed"] += 1
        else:
            by_dist[key]["failed"] += 1

    # Print distribution results
    for dist, counts in by_dist.items():
        print(
            f"  {dist}: {counts['passed']}/{counts['total']} passed ({counts['passed'] / counts['total'] * 100:.1f}%)"
        )

    print("\nResults by architecture:")
    # Group by architecture
    by_arch = {}
    for r in results:
        key = r["architecture"]
        if key not in by_arch:
            by_arch[key] = {"total": 0, "passed": 0, "failed": 0}
        by_arch[key]["total"] += 1
        if r["status"] == "PASS":
            by_arch[key]["passed"] += 1
        else:
            by_arch[key]["failed"] += 1

    # Print architecture results
    for arch, counts in by_arch.items():
        print(
            f"  {arch}: {counts['passed']}/{counts['total']} passed ({counts['passed'] / counts['total'] * 100:.1f}%)"
        )

    print("\nResults by libc variant:")
    # Group by libc variant
    by_libc = {}
    for r in results:
        key = r["libc_variant"]
        if key not in by_libc:
            by_libc[key] = {"total": 0, "passed": 0, "failed": 0}
        by_libc[key]["total"] += 1
        if r["status"] == "PASS":
            by_libc[key]["passed"] += 1
        else:
            by_libc[key]["failed"] += 1

    # Print libc variant results
    for libc, counts in by_libc.items():
        print(
            f"  {libc}: {counts['passed']}/{counts['total']} passed ({counts['passed'] / counts['total'] * 100:.1f}%)"
        )

    print("\nFailed tests:")
    failed_tests = [r for r in results if r["status"] == "FAIL"]
    if failed_tests:
        for r in failed_tests:
            print(
                f"  {r['distribution']} {r['version']} on {r['architecture']} with libc variant: {r['libc_variant']}"
            )
    else:
        print("  None")

    # Save summary to file
    summary_file = Path("results/zip_test_summary.json")
    summary_file.parent.mkdir(exist_ok=True)

    summary_data = {
        "timestamp": datetime.now().isoformat(),
        "total_tests": total,
        "passed": passed,
        "failed": failed,
        "success_rate": passed / total * 100,
        "results_by_distribution": by_dist,
        "results_by_architecture": by_arch,
        "results_by_libc_variant": by_libc,
        "failed_tests": [
            {
                "distribution": r["distribution"],
                "version": r["version"],
                "architecture": r["architecture"],
                "libc_variant": r["libc_variant"],
            }
            for r in failed_tests
        ],
        "detailed_results": results,
    }

    with open(summary_file, "w") as f:
        json.dump(summary_data, f, indent=2)

    print(f"\nSummary saved to {summary_file}")


def main():
    """Main function."""
    args = parse_args()

    # Parse distributions
    distributions = DEFAULT_DISTRIBUTIONS
    if args.distributions:
        distributions = []
        for dist_str in args.distributions.split(","):
            if ":" in dist_str:
                dist, version = dist_str.split(":", 1)
                distributions.append((dist, version))
            else:
                # If no version specified, use all default versions for this distribution
                dist = dist_str
                versions = [v for d, v in DEFAULT_DISTRIBUTIONS if d == dist]
                if not versions:
                    print(
                        f"Warning: No default versions found for {dist}, using 'latest'"
                    )
                    versions = ["latest"]
                for version in versions:
                    distributions.append((dist, version))

    # Parse architectures
    architectures = DEFAULT_ARCHITECTURES
    if args.architectures:
        architectures = args.architectures.split(",")

    # Parse libc variants
    libc_variants = DEFAULT_LIBC_VARIANTS
    if args.libc_variants:
        libc_variants = args.libc_variants.split(",")

    # Create results directory
    Path("results").mkdir(exist_ok=True)

    # Run tests
    results = []
    total_tests = len(distributions) * len(architectures) * len(libc_variants)
    print(f"Running {total_tests} ZIP installation tests...")

    for i, (distribution, version) in enumerate(distributions):
        for j, architecture in enumerate(architectures):
            for k, libc_variant in enumerate(libc_variants):
                test_num = (
                    i * len(architectures) * len(libc_variants)
                    + j * len(libc_variants)
                    + k
                    + 1
                )
                print(f"\nTest {test_num}/{total_tests}")

                result = run_test(
                    distribution, version, architecture, libc_variant, args.runtime
                )
                results.append(result)

                # Save individual test result
                result_file = Path(
                    f"results/zip_test_{distribution}_{version}_{architecture}_{libc_variant}.json"
                )
                with open(result_file, "w") as f:
                    json.dump(result, f, indent=2)

    # Generate summary (default behavior unless --no-summary is specified)
    if not args.no_summary:
        generate_summary(results)

    # Return non-zero exit code if any test failed
    if any(r["status"] == "FAIL" for r in results):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
