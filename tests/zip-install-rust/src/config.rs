use serde::{Deserialize, Serialize};

/// Standard timeout for all operations (5 minutes)
pub const TIMEOUT_SECS: u64 = 300;

/// Common packages needed for all distributions
pub const COMMON_PACKAGES: &str = "unzip sudo";

/// Distribution-specific curl packages
pub const CURL_PACKAGES: &[(&str, &str)] = &[
    ("ubuntu", "curl"),
    ("debian", "curl"),
    ("amazonlinux", "curl-minimal"),
    ("rocky", "curl"),
    ("fedora", "curl"),
    ("alpine", "curl"),
];

/// Configuration for a Linux distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionConfig {
    pub name: String,
    pub version: String,
    pub architectures: Vec<String>,
    pub libc_variants: Vec<String>,
}

/// Parameters for a specific distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionParams {
    pub base_image: String,
    pub package_manager: String,
    pub sudo_group: String,
    pub has_selinux: bool,
    pub extra_setup: Option<String>,
    pub package_install_flags: Option<String>,
}

/// Get all supported distributions for testing
pub fn get_distributions() -> Vec<DistributionConfig> {
    vec![
        // Ubuntu
        DistributionConfig {
            name: "ubuntu".to_string(),
            version: "24.04".to_string(),
            architectures: vec!["x86_64".to_string(), "arm64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        DistributionConfig {
            name: "ubuntu".to_string(),
            version: "22.04".to_string(),
            architectures: vec!["x86_64".to_string(), "arm64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        DistributionConfig {
            name: "ubuntu".to_string(),
            version: "20.04".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        
        // Debian
        DistributionConfig {
            name: "debian".to_string(),
            version: "12".to_string(),
            architectures: vec!["x86_64".to_string(), "arm64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        DistributionConfig {
            name: "debian".to_string(),
            version: "11".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        
        // Amazon Linux
        DistributionConfig {
            name: "amazonlinux".to_string(),
            version: "2023".to_string(),
            architectures: vec!["x86_64".to_string(), "arm64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        DistributionConfig {
            name: "amazonlinux".to_string(),
            version: "2".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["glibc".to_string()],
        },
        
        // Rocky Linux
        DistributionConfig {
            name: "rocky".to_string(),
            version: "9".to_string(),
            architectures: vec!["x86_64".to_string(), "arm64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        
        // Fedora
        DistributionConfig {
            name: "fedora".to_string(),
            version: "39".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        DistributionConfig {
            name: "fedora".to_string(),
            version: "38".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["glibc".to_string(), "musl".to_string()],
        },
        
        // Alpine
        DistributionConfig {
            name: "alpine".to_string(),
            version: "3.19".to_string(),
            architectures: vec!["x86_64".to_string()],
            libc_variants: vec!["musl".to_string()],
        },
    ]
}

/// Get distribution parameters for a specific distribution
pub fn get_distribution_params(distro: &str) -> DistributionParams {
    match distro {
        "ubuntu" | "debian" => DistributionParams {
            base_image: format!("{}:{{version}}", distro),
            package_manager: "apt-get update && apt-get install -y".to_string(),
            sudo_group: "sudo".to_string(),
            has_selinux: false,
            extra_setup: None,
            package_install_flags: Some("--no-install-recommends".to_string()),
        },
        "amazonlinux" => DistributionParams {
            base_image: "amazonlinux:{version}".to_string(),
            package_manager: "yum install -y".to_string(),
            sudo_group: "wheel".to_string(),
            has_selinux: true,
            extra_setup: None,
            package_install_flags: Some("--allowerasing --skip-broken".to_string()),
        },
        "rocky" | "fedora" => DistributionParams {
            base_image: if distro == "rocky" {
                "rockylinux:{version}".to_string()
            } else {
                "fedora:{version}".to_string()
            },
            package_manager: "dnf install -y".to_string(),
            sudo_group: "wheel".to_string(),
            has_selinux: true,
            extra_setup: None,
            package_install_flags: Some("--allowerasing --nobest".to_string()),
        },
        "alpine" => DistributionParams {
            base_image: "alpine:{version}".to_string(),
            package_manager: "apk add".to_string(),
            sudo_group: "wheel".to_string(),
            has_selinux: false,
            extra_setup: Some("mkdir -p /run/openrc && touch /run/openrc/softlevel".to_string()),
            package_install_flags: None,
        },
        _ => panic!("Unsupported distribution: {}", distro),
    }
}

/// Get extra packages needed for a specific distribution
pub fn get_extra_packages(distro: &str) -> &'static str {
    match distro {
        "ubuntu" | "debian" => "apt-utils",
        "amazonlinux" | "rocky" | "fedora" => "shadow-utils || true",
        "alpine" => "shadow bash",
        _ => "",
    }
}

/// Get curl package for a specific distribution
pub fn get_curl_package(distro: &str) -> &'static str {
    CURL_PACKAGES
        .iter()
        .find(|(name, _)| *name == distro)
        .map(|(_, package)| *package)
        .unwrap_or("curl")
}
