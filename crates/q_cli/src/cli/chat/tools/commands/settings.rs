use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the settings command and its subcommands
pub struct SettingsCommand {
    subcommand: Option<String>,
}

impl SettingsCommand {
    // Subcommand constants with their descriptions
    const LIST: (&'static str, &'static str) = ("list", "Listing current settings");
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 2] = [Self::SET.0, Self::RESET.0];
    const RESET: (&'static str, &'static str) = ("reset", "Resetting settings to default");
    const SET: (&'static str, &'static str) = ("set", "Setting configuration");
    // Map of subcommand names to their descriptions
    const SUBCOMMAND_DESCRIPTIONS: [(&'static str, &'static str); 3] = [Self::LIST, Self::SET, Self::RESET];
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 3] = [Self::LIST.0, Self::SET.0, Self::RESET.0];

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
            .map_or("Executing settings command", |(_, desc)| *desc)
    }
}

impl CommandBehavior for SettingsCommand {
    fn requires_acceptance(&self) -> bool {
        // Only certain subcommands require user confirmation
        self.is_modifying_subcommand()
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the subcommand is one we support
        match &self.subcommand {
            Some(_) if self.is_valid_subcommand() => Ok(()),
            Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for settings command", subcmd)),
            None => Err(eyre::eyre!("Subcommand is required for settings command")),
        }
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        let description = match &self.subcommand {
            Some(subcmd) => Self::get_subcommand_description(subcmd).to_string(),
            None => "Executing settings command".to_string(),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    fn format_command(&self) -> String {
        match &self.subcommand {
            Some(subcmd) => format!("/settings {}", subcmd),
            None => "/settings".to_string(),
        }
    }
}
