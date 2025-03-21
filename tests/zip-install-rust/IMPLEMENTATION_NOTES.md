# Implementation Notes for Amazon Q Developer CLI ZIP Installation Tests

## Overview

This document provides implementation notes for the Rust-based ZIP installation tests for the Amazon Q Developer CLI. It outlines the key issues encountered during development and suggests solutions.

## Key Issues and Solutions

### 1. Workspace Integration

The crate has been added to the workspace in the root `Cargo.toml`, but there are several issues to address:

```toml
# Add to workspace members in root Cargo.toml
members = [
    # ...existing members
    "tests/zip-install-rust",
]
```

### 2. Bollard Docker API Version Mismatch

The Bollard crate (v0.15.0) has API differences compared to the latest version. The following changes are needed:

- Update Docker connection:
  ```rust
  // Change from:
  Docker::connect_with_socket("/run/finch/finch.sock", 120, None)
  
  // To:
  Docker::connect_with_socket_defaults().or_else(|_| {
      Docker::connect_with_local_defaults()
  })
  ```

- Fix build_image stream handling:
  ```rust
  // Change from:
  let mut build_stream = self.docker.build_image(build_options, None, Some(build_context.as_str()))
  
  // To:
  let mut build_stream = self.docker.build_image(build_options, None, Some(build_context.into()))
  ```

- Fix exec command handling:
  ```rust
  // Change from:
  let output = match self.docker.start_exec(&exec.id, None::<StartExecOptions>) {
      Ok(output) => output,
      Err(e) => return Err(anyhow::anyhow!("Failed to start exec: {}", e)),
  };
  
  // To:
  let output = self.docker.start_exec(&exec.id, None::<StartExecOptions>)
      .await
      .context("Failed to start exec")?;
  ```

### 3. Constants in Rust

The `DISTRIBUTIONS` constant can't use dynamic values like `to_string()` or `vec![]` in a const context. Options:

1. Use a function instead:
   ```rust
   pub fn get_distributions() -> Vec<DistributionConfig> {
       vec![
           DistributionConfig {
               name: "ubuntu".to_string(),
               // ...
           },
           // ...
       ]
   }
   ```

2. Use static strings and convert when needed:
   ```rust
   pub const DISTRIBUTION_NAMES: &[&str] = &["ubuntu", "debian", /* ... */];
   pub const DISTRIBUTION_VERSIONS: &[&str] = &["24.04", "22.04", /* ... */];
   // ...
   
   pub fn get_distribution_config(name: &str, version: &str) -> Option<DistributionConfig> {
       // Look up and construct the config
   }
   ```

### 4. HashMap Type Mismatch

Fix the HashMap type mismatch in container.rs:

```rust
// Change from:
let mut args = HashMap::new();
args.insert("LIBC_VARIANT", &options.libc_variant);

// To:
let mut args = HashMap::new();
args.insert("LIBC_VARIANT", options.libc_variant.as_str());
```

### 5. Container Name Type

Fix the container name type:

```rust
// Change from:
let container_name = format!("q-test-{}", uuid::Uuid::new_v4());

// To:
let container_name = format!("q-test-{}", uuid::Uuid::new_v4().to_string());
```

## Recommended Next Steps

1. Fix the Bollard API usage to match the version being used
2. Replace the constant `DISTRIBUTIONS` with a function that returns the configurations
3. Fix type mismatches in the HashMap and container name
4. Update the Docker connection logic to work with both Finch and Docker
5. Add proper error handling with context

Once these issues are addressed, the implementation should work correctly and integrate well with the existing codebase.
