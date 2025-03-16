#!/usr/bin/env python3
"""
Simple script to download Amazon Q Developer CLI installers from a base URL or S3 bucket
and place them in the correct folders.

Usage: python download_installers.py <base_url | s3_url>
Examples:
  python download_installers.py https://desktop-release.codewhisperer.us-east-1.amazonaws.com/latest/
  python download_installers.py s3://my-bucket/amazon-q/latest/
"""

import os
import subprocess
import sys
from pathlib import Path
from urllib.parse import urljoin


def main():
    # Check for base URL argument
    if len(sys.argv) != 2:
        print("Usage: python download_installers.py <base_url | s3_url>")
        print("Examples:")
        print(
            "  python download_installers.py https://desktop-release.codewhisperer.us-east-1.amazonaws.com/latest/"
        )
        print("  python download_installers.py s3://my-bucket/amazon-q/latest/")
        return 1

    # Get base URL from command line argument
    base_url = sys.argv[1]
    if not base_url.endswith("/"):
        base_url += "/"

    # Determine if we're using S3 or HTTP
    is_s3 = base_url.startswith("s3://")

    # Find the bundle directory relative to the script
    script_dir = Path(__file__).resolve().parent
    bundle_dir = script_dir.parent / "bundle"

    print(f"Using {'S3 path' if is_s3 else 'base URL'}: {base_url}")
    print(f"Using bundle directory: {bundle_dir}")

    # File mappings: source filename -> (destination folder, destination filename)
    file_mappings = {
        # AppImage files
        "amazon-q.appimage": ("appimage", "amazon-q-developer-cli-x86_64.AppImage"),
        # DEB files
        "amazon-q.deb": ("deb", "amazon-q-developer-cli_amd64.deb"),
        # ZIP files
        "q-x86_64-linux.zip": ("zip", "amazon-q-developer-cli-x86_64-linux.zip"),
        "q-aarch64-linux.zip": ("zip", "amazon-q-developer-cli-aarch64-linux.zip"),
        "q-x86_64-linux-musl.zip": (
            "zip",
            "amazon-q-developer-cli-x86_64-linux-musl.zip",
        ),
        "q-aarch64-linux-musl.zip": (
            "zip",
            "amazon-q-developer-cli-aarch64-linux-musl.zip",
        ),
    }

    # Clean destination directories
    print("Cleaning destination directories...")
    for folder in set(mapping[0] for mapping in file_mappings.values()):
        folder_path = bundle_dir / folder
        if folder_path.exists():
            print(f"Cleaning {folder_path}")
            for file in folder_path.glob("*"):
                file.unlink()
        else:
            folder_path.mkdir(parents=True, exist_ok=True)

    # Download files
    success_count = 0
    failure_count = 0

    for source_file, (dest_folder, dest_file) in file_mappings.items():
        # Construct the full URL/path and destination path
        if is_s3:
            source_path = urljoin(base_url, source_file).replace("s3:/", "s3:")
        else:
            source_path = urljoin(base_url, source_file)

        dest_path = bundle_dir / dest_folder / dest_file

        # Create parent directory if it doesn't exist
        os.makedirs(os.path.dirname(dest_path), exist_ok=True)

        # Download the file
        print(f"Downloading {source_path} to {dest_path}")
        try:
            if is_s3:
                # Use AWS CLI for S3 downloads
                subprocess.run(
                    ["aws", "s3", "cp", source_path, str(dest_path)], check=True
                )
            else:
                # Use curl for HTTP downloads
                subprocess.run(
                    ["curl", "-L", "-s", "-o", str(dest_path), source_path], check=True
                )

            # Make AppImage files executable
            if dest_path.suffix == ".AppImage":
                os.chmod(dest_path, 0o755)

            success_count += 1
        except Exception as e:
            print(f"Error downloading {source_path}: {e}", file=sys.stderr)
            failure_count += 1

    # Print summary
    print("\nDownload Summary:")
    print(f"  Successfully downloaded: {success_count} files")
    print(f"  Failed downloads: {failure_count} files")

    # List downloaded files with sizes
    print("\nDownloaded files:")
    for folder in set(mapping[0] for mapping in file_mappings.values()):
        folder_path = bundle_dir / folder
        if folder_path.exists():
            for file_path in folder_path.glob("*"):
                if file_path.is_file():
                    size_mb = file_path.stat().st_size / (1024 * 1024)
                    print(
                        f"  {file_path.relative_to(bundle_dir.parent)} ({size_mb:.1f} MB)"
                    )

    return 0 if failure_count == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
