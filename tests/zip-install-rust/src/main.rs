use std::path::PathBuf;

use anyhow::Result;
use clap::{
    Parser,
    Subcommand,
};
use log::{
    LevelFilter,
    error,
    info,
};
use zip_install_test::{
    DockerfileGenerator,
    TestRunner,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Keep containers after tests (don't clean up)
    #[arg(short, long)]
    keep_containers: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run tests for all distributions
    TestAll {
        /// Path to directory containing ZIP files
        #[arg(short, long)]
        zip_dir: PathBuf,
    },
    /// Run tests for a specific distribution
    Test {
        /// Path to directory containing ZIP files
        #[arg(short, long)]
        zip_dir: PathBuf,

        /// Distribution name
        #[arg(short, long)]
        distro: String,

        /// Distribution version
        #[arg(short, long)]
        version: String,

        /// Architecture
        #[arg(short, long)]
        arch: String,

        /// Libc variant
        #[arg(short, long)]
        libc: String,
    },
    /// Generate a Dockerfile for a specific distribution
    GenerateDockerfile {
        /// Distribution name
        #[arg(short, long)]
        distro: String,

        /// Distribution version
        #[arg(short, long)]
        version: String,

        /// Architecture
        #[arg(short, long)]
        arch: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::new().filter_level(LevelFilter::Info).init();

    let cli = Cli::parse();

    // Check if Docker/Finch/Podman is available
    match zip_install_test::check_container_runtime() {
        Ok(runtime) => info!("Using container runtime: {}", runtime),
        Err(e) => {
            error!("{}", e);
            error!("Please install and start Docker Desktop, Podman, or Finch before running tests.");
            std::process::exit(1);
        },
    }

    match cli.command {
        Commands::TestAll { zip_dir } => {
            run_all_tests(zip_dir, cli.keep_containers).await?;
        },
        Commands::Test {
            zip_dir,
            distro,
            version,
            arch,
            libc,
        } => {
            run_test(zip_dir, &distro, &version, &arch, &libc, cli.keep_containers).await?;
        },
        Commands::GenerateDockerfile { distro, version, arch } => {
            let dockerfile = DockerfileGenerator::generate(&distro, &version, &arch)?;
            println!("{}", dockerfile);
        },
    }

    Ok(())
}

async fn run_all_tests(zip_dir: PathBuf, keep_containers: bool) -> Result<()> {
    info!("Running tests for all distributions");

    let runner = TestRunner::new(keep_containers);
    let mut success_count = 0;
    let mut failure_count = 0;

    // Use the function to get distributions instead of the constant
    for dist_config in zip_install_test::get_distributions() {
        for arch in &dist_config.architectures {
            for libc in &dist_config.libc_variants {
                let result = runner
                    .run_test(&dist_config.name, &dist_config.version, arch, libc, &zip_dir)
                    .await;

                match result {
                    Ok(_) => {
                        info!(
                            "✅ Test passed: {}-{}-{}-{}",
                            dist_config.name, dist_config.version, arch, libc
                        );
                        success_count += 1;
                    },
                    Err(e) => {
                        error!(
                            "❌ Test error for {}-{}-{}-{}: {}",
                            dist_config.name, dist_config.version, arch, libc, e
                        );
                        failure_count += 1;
                    },
                }
            }
        }
    }

    info!("Test summary: {} passed, {} failed", success_count, failure_count);

    if failure_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

async fn run_test(
    zip_dir: PathBuf,
    distro: &str,
    version: &str,
    arch: &str,
    libc: &str,
    keep_containers: bool,
) -> Result<()> {
    info!("Running test for {}-{}-{}-{}", distro, version, arch, libc);

    let runner = TestRunner::new(keep_containers);
    runner.run_test(distro, version, arch, libc, &zip_dir).await?;

    info!("✅ Test passed");

    Ok(())
}
