#!/bin/bash
set -e

# Default values
DISTRO=${1:-"ubuntu"}
VERSION=${2:-"22.04"}
ARCH=${3:-"x86_64"}
LIBC=${4:-"glibc"}
TEST_TYPE=${5:-"both"}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed or not in PATH"
    exit 1
fi

# Validate test type
if [[ "$TEST_TYPE" != "root" && "$TEST_TYPE" != "user" && "$TEST_TYPE" != "both" ]]; then
    echo "Error: Invalid test type. Must be 'root', 'user', or 'both'"
    echo "Usage: $0 [distro] [version] [arch] [libc] [test_type]"
    exit 1
fi

# Run the test
echo "Running $TEST_TYPE installation test for $DISTRO $VERSION ($ARCH, $LIBC)..."
RUST_LOG=info cargo run --bin run-test -- --distro "$DISTRO" --version "$VERSION" --arch "$ARCH" --libc "$LIBC" --test-type "$TEST_TYPE" --keep-containers
