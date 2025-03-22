use anyhow::Result;
use test_case::test_case;
use zip_install_test::{
    DockerfileGenerator,
    TestRunner,
    get_distributions,
};

mod common;

// Skip these tests if Docker/container runtime is not available
// or if test data is not available
macro_rules! skip_if_no_container_runtime {
    () => {
        if !common::container_runtime_available() {
            eprintln!("Skipping test: No container runtime available");
            return Ok(());
        }
        if !common::test_data_available() {
            eprintln!("Skipping test: No test data available");
            return Ok(());
        }
    };
}

#[tokio::test]
async fn test_ubuntu_latest_root() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_root_install_test("ubuntu", "24.04", "x86_64", "glibc", &zip_dir)
        .await?;

    assert!(result, "Ubuntu 24.04 root installation test failed");
    Ok(())
}

#[tokio::test]
async fn test_ubuntu_latest_user() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_user_install_test("ubuntu", "24.04", "x86_64", "glibc", &zip_dir)
        .await?;

    assert!(result, "Ubuntu 24.04 user installation test failed");
    Ok(())
}

#[tokio::test]
async fn test_amazonlinux_latest_root() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_root_install_test("amazonlinux", "2023", "x86_64", "glibc", &zip_dir)
        .await?;

    assert!(result, "Amazon Linux 2023 root installation test failed");
    Ok(())
}

#[tokio::test]
async fn test_amazonlinux_latest_user() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_user_install_test("amazonlinux", "2023", "x86_64", "glibc", &zip_dir)
        .await?;

    assert!(result, "Amazon Linux 2023 user installation test failed");
    Ok(())
}

#[tokio::test]
async fn test_alpine_latest_root() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_root_install_test("alpine", "3.19", "x86_64", "musl", &zip_dir)
        .await?;

    assert!(result, "Alpine 3.19 root installation test failed");
    Ok(())
}

#[tokio::test]
async fn test_alpine_latest_user() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let runner = TestRunner::new(&zip_dir).await?;
    let result = runner
        .run_user_install_test("alpine", "3.19", "x86_64", "musl", &zip_dir)
        .await?;

    assert!(result, "Alpine 3.19 user installation test failed");
    Ok(())
}

// Test all distributions defined in the crate
#[tokio::test]
async fn test_all_distributions_root() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let mut success_count = 0;
    let mut failure_count = 0;

    for dist in get_distributions() {
        // Only test x86_64 to save time
        let arch = "x86_64";
        if !dist.architectures.contains(&arch.to_string()) {
            continue;
        }

        for libc in &dist.libc_variants {
            let runner = TestRunner::new(&zip_dir).await?;
            match runner
                .run_root_install_test(&dist.name, &dist.version, arch, libc, &zip_dir)
                .await
            {
                Ok(true) => success_count += 1,
                _ => failure_count += 1,
            }
        }
    }

    assert_eq!(failure_count, 0, "{} root installation tests failed", failure_count);
    assert!(success_count > 0, "No root installation tests were run");

    Ok(())
}

// Test all distributions defined in the crate
#[tokio::test]
async fn test_all_distributions_user() -> Result<()> {
    skip_if_no_container_runtime!();

    let zip_dir = common::test_data_dir();
    let mut success_count = 0;
    let mut failure_count = 0;

    for dist in get_distributions() {
        // Only test x86_64 to save time
        let arch = "x86_64";
        if !dist.architectures.contains(&arch.to_string()) {
            continue;
        }

        for libc in &dist.libc_variants {
            let runner = TestRunner::new(&zip_dir).await?;
            match runner
                .run_user_install_test(&dist.name, &dist.version, arch, libc, &zip_dir)
                .await
            {
                Ok(true) => success_count += 1,
                _ => failure_count += 1,
            }
        }
    }

    assert_eq!(failure_count, 0, "{} user installation tests failed", failure_count);
    assert!(success_count > 0, "No user installation tests were run");

    Ok(())
}

#[test_case("ubuntu", "22.04", "x86_64", "FROM ubuntu:22.04"; "ubuntu 22.04")]
#[test_case("alpine", "3.19", "x86_64", "FROM alpine:3.19"; "alpine 3.19")]
#[test_case("amazonlinux", "2023", "x86_64", "FROM amazonlinux:2023"; "amazonlinux 2023")]
fn test_dockerfile_generation(distro: &str, version: &str, arch: &str, expected: &str) -> Result<()> {
    let dockerfile = DockerfileGenerator::generate(distro, version, arch)?;

    assert!(dockerfile.contains(expected));
    assert!(dockerfile.contains("ARG LIBC_VARIANT=glibc"));
    assert!(dockerfile.contains("ENV LIBC_VARIANT=${LIBC_VARIANT}"));

    Ok(())
}

#[test_case("ubuntu", "22.04", "x86_64", "glibc", "ubuntu-22.04-x86_64-glibc"; "ubuntu glibc")]
#[test_case("alpine", "3.19", "x86_64", "musl", "alpine-3.19-x86_64-musl"; "alpine musl")]
fn test_tag_generation(distro: &str, version: &str, arch: &str, libc: &str, expected: &str) {
    let tag = DockerfileGenerator::generate_tag(distro, version, arch, libc);
    assert_eq!(tag, expected);
}
