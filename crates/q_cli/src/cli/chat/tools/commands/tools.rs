use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the tools command and its subcommands
pub struct ToolsCommand {
    subcommand: Option<String>,
}

impl ToolsCommand {
    const DISABLE: &'static str = "disable";
    const ENABLE: &'static str = "enable";
    const INFO: &'static str = "info";
    const INSTALL: &'static str = "install";
    // Subcommand constants
    const LIST: &'static str = "list";
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 5] = [
        Self::ENABLE,
        Self::DISABLE,
        Self::INSTALL,
        Self::UNINSTALL,
        Self::UPDATE,
    ];
    const UNINSTALL: &'static str = "uninstall";
    const UPDATE: &'static str = "update";
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 7] = [
        Self::LIST,
        Self::ENABLE,
        Self::DISABLE,
        Self::INSTALL,
        Self::UNINSTALL,
        Self::UPDATE,
        Self::INFO,
    ];

    pub fn new(subcommand: Option<&str>) -> Self {
        Self {
            subcommand: subcommand.map(String::from),
        }
    }

    /// Check if the subcommand is one that modifies state and requires confirmation
    fn is_modifying_subcommand(&self) -> bool {
        self.subcommand
            .as_deref()
            .is_some_and(|s| Self::MODIFYING_SUBCOMMANDS.contains(&s))
    }

    /// Check if the subcommand is valid
    fn is_valid_subcommand(&self) -> bool {
        self.subcommand
            .as_deref()
            .is_some_and(|s| Self::VALID_SUBCOMMANDS.contains(&s))
    }
}

impl CommandBehavior for ToolsCommand {
    fn requires_acceptance(&self) -> bool {
        // Only certain subcommands require user confirmation
        self.is_modifying_subcommand()
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the subcommand is one we support
        match &self.subcommand {
            Some(_) if self.is_valid_subcommand() => Ok(()),
            Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for tools command", subcmd)),
            None => Err(eyre::eyre!("Subcommand is required for tools command")),
        }
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        let description = match self.subcommand.as_deref() {
            Some(Self::LIST) => "Listing available tools".to_string(),
            Some(Self::ENABLE) => "Enabling tool".to_string(),
            Some(Self::DISABLE) => "Disabling tool".to_string(),
            Some(Self::INSTALL) => "Installing tool".to_string(),
            Some(Self::UNINSTALL) => "Uninstalling tool".to_string(),
            Some(Self::UPDATE) => "Updating tools".to_string(),
            Some(Self::INFO) => "Showing information about tool".to_string(),
            Some(subcmd) => format!("Executing tools {}", subcmd),
            None => "Executing tools command".to_string(),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    fn format_command(&self) -> String {
        match &self.subcommand {
            Some(subcmd) => format!("/tools {}", subcmd),
            None => "/tools".to_string(),
        }
    }
}
