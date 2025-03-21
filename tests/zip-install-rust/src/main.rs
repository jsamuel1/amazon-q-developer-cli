use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::{
    Parser,
    Subcommand,
};
use log::info;
use zip_install_test::{
    TestRunner,
    get_distributions,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a test for a specific distribution
    RunTest {
        /// Distribution name
        #[arg(short, long)]
        distro: String,

        /// Distribution version
        #[arg(short, long)]
        version: String,

        /// Architecture
        #[arg(short, long, default_value = "x86_64")]
        arch: String,

        /// Libc variant
        #[arg(short, long, default_value = "glibc")]
        libc: String,

        /// Path to directory containing ZIP files
        #[arg(short, long)]
        zip_dir: Option<PathBuf>,

        /// Keep containers after test
        #[arg(short, long)]
        keep_containers: bool,

        /// Test type: root, user, or both
        #[arg(short, long, default_value = "both")]
        test_type: String,
    },

    /// List all supported distributions
    ListDistributions,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::RunTest {
            distro,
            version,
            arch,
            libc,
            zip_dir,
            keep_containers,
            test_type,
        } => {
            let zip_dir = zip_dir.clone().unwrap_or_else(|| {
                let dir = PathBuf::from("test_data");
                if !dir.exists() {
                    eprintln!("Error: test_data directory not found");
                    process::exit(1);
                }
                dir
            });

            let runner = TestRunner::new(&zip_dir).await?.with_keep_containers(*keep_containers);

            match test_type.as_str() {
                "root" => {
                    info!("Running root installation test for {distro} {version} ({arch}, {libc})");
                    let result = runner
                        .run_root_install_test(distro, version, arch, libc, &zip_dir)
                        .await?;
                    if result {
                        info!("Root installation test passed!");
                        Ok(())
                    } else {
                        eprintln!("Root installation test failed!");
                        process::exit(1);
                    }
                },
                "user" => {
                    info!("Running user installation test for {distro} {version} ({arch}, {libc})");
                    let result = runner
                        .run_user_install_test(distro, version, arch, libc, &zip_dir)
                        .await?;
                    if result {
                        info!("User installation test passed!");
                        Ok(())
                    } else {
                        eprintln!("User installation test failed!");
                        process::exit(1);
                    }
                },
                "both" => {
                    info!("Running both installation tests for {distro} {version} ({arch}, {libc})");
                    let result = runner.run_test(distro, version, arch, libc, &zip_dir).await?;
                    if result {
                        info!("All installation tests passed!");
                        Ok(())
                    } else {
                        eprintln!("Installation tests failed!");
                        process::exit(1);
                    }
                },
                _ => {
                    info!("Running both installation tests for {distro} {version} ({arch}, {libc})");
                    let result = runner.run_test(distro, version, arch, libc, &zip_dir).await?;
                    if result {
                        info!("All installation tests passed!");
                        Ok(())
                    } else {
                        eprintln!("Installation tests failed!");
                        process::exit(1);
                    }
                },
            }
        },
        Commands::ListDistributions => {
            println!("Supported distributions:");
            for dist in get_distributions() {
                println!("- {} {}", dist.name, dist.version);
                println!("  Architectures: {}", dist.architectures.join(", "));
                println!("  Libc variants: {}", dist.libc_variants.join(", "));
            }
            Ok(())
        },
    }
}
