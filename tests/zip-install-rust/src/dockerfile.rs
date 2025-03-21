use anyhow::{
    Result,
    anyhow,
};

/// Generator for Dockerfiles
pub struct DockerfileGenerator;

impl DockerfileGenerator {
    /// Generate a Dockerfile for the given distribution
    pub fn generate(distro: &str, version: &str, _arch: &str) -> Result<String> {
        let base_image = format!("{}:{}", distro, version);

        // Check if the distribution is supported
        match distro {
            "ubuntu" | "debian" | "amazonlinux" | "alpine" | "fedora" | "centos" | "rockylinux" => {},
            _ => return Err(anyhow!("Unsupported distribution: {}", distro)),
        }

        let dockerfile = format!(
            r#"FROM {}

ARG LIBC_VARIANT=glibc
ENV LIBC_VARIANT=$LIBC_VARIANT

# Install dependencies
RUN if command -v apt-get &> /dev/null; then \
      apt-get update && apt-get install -y curl unzip sudo; \
    elif command -v yum &> /dev/null; then \
      yum install -y curl unzip sudo; \
    elif command -v apk &> /dev/null; then \
      apk add --no-cache curl unzip sudo; \
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
"#,
            base_image
        );

        Ok(dockerfile)
    }

    /// Generate a tag for the Docker image
    pub fn generate_tag(distro: &str, version: &str, arch: &str, libc: &str) -> String {
        format!("{}-{}-{}-{}", distro, version, arch, libc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_dockerfile() {
        let dockerfile = DockerfileGenerator::generate("ubuntu", "22.04", "x86_64").unwrap();
        assert!(dockerfile.contains("FROM ubuntu:22.04"));
        assert!(dockerfile.contains("ARG LIBC_VARIANT=glibc"));
    }

    #[test]
    fn test_generate_tag() {
        let tag = DockerfileGenerator::generate_tag("ubuntu", "22.04", "x86_64", "glibc");
        assert_eq!(tag, "ubuntu-22.04-x86_64-glibc");
    }

    #[test]
    fn test_unsupported_distro() {
        let result = DockerfileGenerator::generate("unsupported", "1.0", "x86_64");
        assert!(result.is_err());
    }
}
