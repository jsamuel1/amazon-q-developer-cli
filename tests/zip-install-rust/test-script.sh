#!/bin/bash
set -e

echo "=== Testing Amazon Q Developer CLI installation ==="
echo "Distribution: ubuntu:22.04"
echo "Architecture: x86_64"
echo "Libc: glibc"

# Test as root user
echo "=== Testing as root user ==="
cd /amazon-q-developer-cli/bundle
unzip -o amazon-q-developer-cli.zip
ls -la
cd q
./install.sh --force --no-confirm

# Verify installation
which q
q --version

# Test as regular user
echo "=== Testing as regular user ==="
su - quser -c "cd /amazon-q-developer-cli/bundle && unzip -o amazon-q-developer-cli.zip"
su - quser -c "cd /amazon-q-developer-cli/bundle/q && ./install.sh --no-confirm"

# Verify installation
su - quser -c "which q"
su - quser -c "q --version"

echo "=== Test completed successfully ==="
