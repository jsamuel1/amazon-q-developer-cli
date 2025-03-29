use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the tools command and its subcommands
pub struct ToolsCommand {
    subcommand: Option<String>,
}

impl ToolsCommand {
    const DISABLE: (&'static str, &'static str) = ("disable", "Disabling tool");
    const ENABLE: (&'static str, &'static str) = ("enable", "Enabling tool");
    const INFO: (&'static str, &'static str) = ("info", "Showing information about tool");
    const INSTALL: (&'static str, &'static str) = ("install", "Installing tool");
    // Subcommand constants with their descriptions
    const LIST: (&'static str, &'static str) = ("list", "Listing available tools");
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 5] = [
        Self::ENABLE.0,
        Self::DISABLE.0,
        Self::INSTALL.0,
        Self::UNINSTALL.0,
        Self::UPDATE.0,
    ];
    // Map of subcommand names to their descriptions
    const SUBCOMMAND_DESCRIPTIONS: [(&'static str, &'static str); 7] = [
        Self::LIST,
        Self::ENABLE,
        Self::DISABLE,
        Self::INSTALL,
        Self::UNINSTALL,
        Self::UPDATE,
        Self::INFO,
    ];
    const UNINSTALL: (&'static str, &'static str) = ("uninstall", "Uninstalling tool");
    const UPDATE: (&'static str, &'static str) = ("update", "Updating tools");
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 7] = [
        Self::LIST.0,
        Self::ENABLE.0,
        Self::DISABLE.0,
        Self::INSTALL.0,
        Self::UNINSTALL.0,
        Self::UPDATE.0,
        Self::INFO.0,
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

    /// Get the description for a subcommand
    fn get_subcommand_description(subcmd: &str) -> &'static str {
        Self::SUBCOMMAND_DESCRIPTIONS
            .iter()
            .find(|(cmd, _)| *cmd == subcmd)
            .map_or("Executing tools command", |(_, desc)| *desc)
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
        let description = match &self.subcommand {
            Some(subcmd) => Self::get_subcommand_description(subcmd).to_string(),
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
