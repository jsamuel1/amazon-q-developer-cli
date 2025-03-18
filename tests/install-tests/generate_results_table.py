#!/usr/bin/env python3
"""
Script to generate a markdown table of the run_zip_tests results.

This script reads the zip_test_summary.json file and generates a markdown table
with distribution/version on the first column, and architecture and glibc type
along the column headings.

Usage:
    python generate_results_table.py [--input INPUT_FILE] [--output OUTPUT_FILE]
"""

import argparse
import json
import sys


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate a markdown table of the run_zip_tests results"
    )
    parser.add_argument(
        "--input",
        default="results/zip_test_summary.json",
        help="Path to the zip_test_summary.json file",
    )
    parser.add_argument(
        "--output",
        help="Path to the output markdown file. If not specified, prints to stdout",
    )
    return parser.parse_args()


def generate_table(summary_data):
    """Generate a markdown table from the summary data."""
    # Extract unique architectures and libc variants from the actual results
    architectures = set()
    libc_variants = set()

    for result in summary_data["detailed_results"]:
        architectures.add(result["architecture"])
        libc_variants.add(result["libc_variant"])

    # Convert to sorted lists
    architectures = sorted(list(architectures))
    libc_variants = sorted(list(libc_variants))

    # Create column headers
    headers = ["Distribution/Version"]
    for arch in architectures:
        for libc in libc_variants:
            headers.append(f"{arch}/{libc}")

    # Create the table header row
    table = [f"| {' | '.join(headers)} |"]

    # Create the separator row
    separator = ["---"] * len(headers)
    table.append(f"| {' | '.join(separator)} |")

    # Get all distribution/version combinations
    distributions = {}
    for result in summary_data["detailed_results"]:
        dist_key = f"{result['distribution']} {result['version']}"
        if dist_key not in distributions:
            distributions[dist_key] = {"name": dist_key, "results": {}}

        # Create a key for this architecture/libc combination
        arch_libc_key = f"{result['architecture']}/{result['libc_variant']}"
        distributions[dist_key]["results"][arch_libc_key] = result["status"]

    # Sort distributions by name
    sorted_dists = sorted(distributions.values(), key=lambda x: x["name"])

    # Create table rows
    for dist in sorted_dists:
        row = [dist["name"]]
        for arch in architectures:
            for libc in libc_variants:
                key = f"{arch}/{libc}"
                status = dist["results"].get(key, "N/A")
                if status.lower() == "pass":
                    row.append("✅")
                elif status.lower() == "fail":
                    row.append("❌")
                else:
                    row.append("⚪")
        table.append(f"| {' | '.join(row)} |")

    return "\n".join(table)


def main():
    """Main function."""
    args = parse_args()

    # Read the summary data
    try:
        with open(args.input) as f:
            summary_data = json.load(f)
    except FileNotFoundError:
        print(f"Error: File {args.input} not found", file=sys.stderr)
        return 1
    except json.JSONDecodeError:
        print(f"Error: File {args.input} is not valid JSON", file=sys.stderr)
        return 1

    # Generate the table
    table = generate_table(summary_data)

    # Add a title and timestamp
    timestamp = summary_data.get("timestamp", "Unknown")
    title = (
        f"# ZIP Installation Test Results\n\nGenerated from results at: {timestamp}\n\n"
    )

    # Add summary statistics
    total = summary_data.get("total_tests", 0)
    passed = summary_data.get("passed", 0)
    failed = summary_data.get("failed", 0)
    skipped = summary_data.get("skipped", 0)
    success_rate = summary_data.get("success_rate", 0)

    summary = "## Summary\n\n"
    summary += f"- Total tests: {total}\n"
    summary += f"- Passed: {passed}\n"
    summary += f"- Failed: {failed}\n"
    summary += f"- Skipped: {skipped}\n"
    summary += f"- Success rate: {success_rate:.1f}% (excluding skipped tests)\n\n"

    # Add failed tests if any
    if failed > 0 and "failed_tests" in summary_data and summary_data["failed_tests"]:
        summary += "### Failed Tests\n\n"
        for test in summary_data["failed_tests"]:
            summary += f"- {test}\n"
        summary += "\n"

    # Add skipped tests if any
    if (
        skipped > 0
        and "skipped_tests" in summary_data
        and summary_data["skipped_tests"]
    ):
        summary += "### Skipped Tests\n\n"
        for test in summary_data["skipped_tests"]:
            summary += f"- {test}\n"
        summary += "\n"

    # Add detailed information for each test
    summary += "## Detailed Test Results\n\n"
    for result in summary_data["detailed_results"]:
        dist_version = f"{result['distribution']} {result['version']}"
        arch_libc = f"{result['architecture']}/{result['libc_variant']}"
        status = "✅ Passed" if result["status"].lower() == "pass" else "❌ Failed"

        summary += f"### {dist_version} ({arch_libc}): {status}\n\n"
        summary += f"- Test: {result['test']}\n"
        summary += f"- Execution time: {result['execution_time']:.2f} seconds\n"

        if result["status"].lower() == "fail":
            summary += f"- Error: {result.get('error', 'Unknown error')}\n"

            # Add output if available
            if "output" in result:
                summary += f"- Output: ```\n{result['output']}\n```\n"

        # Add install log content if available (for both pass and fail)
        if "install_log_content" in result:
            # Truncate log if it's too long
            log_content = result["install_log_content"]
            if len(log_content) > 2000:
                log_content = (
                    log_content[:1000]
                    + "\n...\n[log truncated]\n...\n"
                    + log_content[-1000:]
                )

            summary += f"- Install Log: ```\n{log_content}\n```\n"

        summary += "\n"

    # Combine everything
    output = title + summary + "## Results Table\n\n" + table

    # Write the output
    if args.output:
        with open(args.output, "w") as f:
            f.write(output)
        print(f"Table written to {args.output}")
    else:
        print(output)

    return 0


if __name__ == "__main__":
    sys.exit(main())
