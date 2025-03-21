#!/bin/bash
set -e

# Configuration
DISTRO="ubuntu"
VERSION="22.04"
ARCH="x86_64"
LIBC="glibc"
ZIP_DIR="./test_data"
KEEP_CONTAINERS=true

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --distro)
      DISTRO="$2"
      shift 2
      ;;
    --version)
      VERSION="$2"
      shift 2
      ;;
    --arch)
      ARCH="$2"
      shift 2
      ;;
    --libc)
      LIBC="$2"
      shift 2
      ;;
    --zip-dir)
      ZIP_DIR="$2"
      shift 2
      ;;
    --no-keep-containers)
      KEEP_CONTAINERS=false
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Validate inputs
if [ ! -d "$ZIP_DIR" ]; then
  echo "Error: ZIP directory not found: $ZIP_DIR"
  exit 1
fi

# Find the appropriate ZIP file
if [ "$LIBC" == "musl" ]; then
  ZIP_FILE="$ZIP_DIR/amazon-q-developer-cli-$ARCH-linux-musl.zip"
else
  ZIP_FILE="$ZIP_DIR/amazon-q-developer-cli-$ARCH-linux.zip"
fi

if [ ! -f "$ZIP_FILE" ]; then
  echo "Error: ZIP file not found: $ZIP_FILE"
  exit 1
fi

echo "Using ZIP file: $ZIP_FILE"

# Create a timestamp for the container name
TIMESTAMP=$(date +%s)
CONTAINER_NAME="q-test-$DISTRO-$VERSION-$ARCH-$LIBC-$TIMESTAMP"

echo "Creating container: $CONTAINER_NAME"

# Create a temporary directory to copy the ZIP file
TEMP_DIR=$(mktemp -d)
cp "$ZIP_FILE" "$TEMP_DIR/amazon-q-developer-cli.zip"
echo "Copied ZIP file to $TEMP_DIR/amazon-q-developer-cli.zip"

# Create a Dockerfile
DOCKERFILE="Dockerfile.$CONTAINER_NAME"
cat > "$DOCKERFILE" << EOF
FROM $DISTRO:$VERSION

# Install dependencies
RUN if command -v apt-get &> /dev/null; then \\
      apt-get update && apt-get install -y curl unzip sudo; \\
    elif command -v yum &> /dev/null; then \\
      yum install -y curl unzip sudo; \\
    elif command -v apk &> /dev/null; then \\
      apk add --no-cache curl unzip sudo; \\
    fi

# Create test user
RUN useradd -m quser || true
RUN echo "quser ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/quser

# Create directories
RUN mkdir -p /amazon-q-developer-cli/bundle
WORKDIR /amazon-q-developer-cli

# Copy the ZIP file
COPY amazon-q-developer-cli.zip /amazon-q-developer-cli/bundle/

# Test script
COPY test-script.sh /amazon-q-developer-cli/test-script.sh
RUN chmod +x /amazon-q-developer-cli/test-script.sh

CMD ["/bin/bash", "/amazon-q-developer-cli/test-script.sh"]
EOF

# Create test script
cat > test-script.sh << EOF
#!/bin/bash
set -e

echo "=== Testing Amazon Q Developer CLI installation ==="
echo "Distribution: $DISTRO:$VERSION"
echo "Architecture: $ARCH"
echo "Libc: $LIBC"

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
EOF

# Check if we're running on ARM64 Mac
if [ "$(uname -m)" == "arm64" ]; then
  # For ARM64 Mac, we need to use --platform=linux/amd64 for x86_64 images
  if [ "$ARCH" == "x86_64" ]; then
    PLATFORM_FLAG="--platform=linux/amd64"
  else
    PLATFORM_FLAG="--platform=linux/arm64"
  fi
else
  # For Intel Mac, we need to use --platform=linux/arm64 for aarch64 images
  if [ "$ARCH" == "aarch64" ]; then
    PLATFORM_FLAG="--platform=linux/arm64"
  else
    PLATFORM_FLAG="--platform=linux/amd64"
  fi
fi

# Copy files to temp directory
cp test-script.sh "$TEMP_DIR/"
cp "$DOCKERFILE" "$TEMP_DIR/"

# Build the image
echo "Building image..."
cd "$TEMP_DIR"
finch build $PLATFORM_FLAG -t "$CONTAINER_NAME" -f "$DOCKERFILE" .

# Run the container
echo "Running container..."
finch run --name "$CONTAINER_NAME" "$CONTAINER_NAME"

# Clean up
if [ "$KEEP_CONTAINERS" = false ]; then
  echo "Cleaning up..."
  finch rm "$CONTAINER_NAME"
  finch rmi "$CONTAINER_NAME"
  rm -rf "$TEMP_DIR"
else
  echo "Container kept for inspection: $CONTAINER_NAME"
  echo "Temporary directory: $TEMP_DIR"
fi

echo "Test completed."
