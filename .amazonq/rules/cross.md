# Cross with Finch for Amazon Q Developer CLI

## Required Commands

To use cross-compilation with Finch in the Amazon Q Developer CLI project:

1. Start Finch:
```bash
finch vm start
```

2. Set environment variables:
```bash
export CROSS_CONTAINER_ENGINE=finch
export AMAZON_Q_BUILD_MUSL=1
```

3. Build for Linux with musl:
```bash
# For x86_64
CROSS_CONTAINER_ENGINE=finch cross build --target x86_64-unknown-linux-musl

# For aarch64
CROSS_CONTAINER_ENGINE=finch cross build --target aarch64-unknown-linux-musl
```

4. Check musl version (for CVE verification):
```bash
CROSS_CONTAINER_ENGINE=finch cross run --target x86_64-unknown-linux-musl -- sh -c "musl-gcc --version || echo 'musl-gcc not found'"
```

## Verification

Verify Finch is working with cross:
```bash
finch ps
```

## Troubleshooting

- Finch not running: `finch vm status` then `finch vm start`
- Environment variables: `echo $CROSS_CONTAINER_ENGINE` should return "finch"
- Finch access: `finch info`
- Permission errors: Check Finch permissions
- ARM64 Mac issues: When using an ARM64 Mac (Apple Silicon), you may encounter "no match for platform in manifest" errors. This happens because the cross-rs container images don't have ARM64 variants. Try the following:
  1. Make sure Rosetta is enabled in your Finch config: `rosetta: true` in `~/.finch/finch.yaml`
  2. Restart Finch VM: `finch vm stop && finch vm start`
  3. If issues persist, consider using Docker Desktop with Rosetta or a remote x86_64 build server

## Using Custom Images

If you're experiencing issues with the pre-built cross-rs images (especially on ARM64 Macs), you can build custom images using the Dockerfiles from the cross-toolchains repository:

1. Clone the cross-toolchains repository:
```bash
git clone https://github.com/cross-rs/cross-toolchains.git
cd cross-toolchains
```

2. Build a custom image for your target:
```bash
# For x86_64-unknown-linux-musl
finch build -t cross-toolchains:x86_64-unknown-linux-musl -f dockerfiles/linux-musl/x86_64.Dockerfile .
```

3. Configure Cross to use your custom image by creating or editing `Cross.toml` in your project root:
```toml
[target.x86_64-unknown-linux-musl]
image = "cross-toolchains:x86_64-unknown-linux-musl"
```

4. Run cross with your custom image:
```bash
CROSS_CONTAINER_ENGINE=finch cross build --target x86_64-unknown-linux-musl
```

Note: Building custom images can take significant time and disk space, but it's often the most reliable solution for ARM64 Mac users.

## Security Note

The musl version used is determined by the cross Docker/Finch image. Check the version when assessing vulnerabilities like CVE-2025-26519.
