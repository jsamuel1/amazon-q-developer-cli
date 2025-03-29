use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the profile command and its subcommands
pub struct ProfileCommand {
    subcommand: Option<String>,
}

impl ProfileCommand {
    const CREATE: (&'static str, &'static str) = ("create", "Creating new profile");
    const DELETE: (&'static str, &'static str) = ("delete", "Deleting profile");
    // Subcommand constants with their descriptions
    const HELP: (&'static str, &'static str) = ("help", "Showing profile help information");
    const LIST: (&'static str, &'static str) = ("list", "Listing available profiles");
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 4] = [Self::CREATE.0, Self::DELETE.0, Self::RENAME.0, Self::SET.0];
    const RENAME: (&'static str, &'static str) = ("rename", "Renaming profile");
    const SET: (&'static str, &'static str) = ("set", "Setting active profile");
    // Map of subcommand names to their descriptions
    const SUBCOMMAND_DESCRIPTIONS: [(&'static str, &'static str); 6] = [
        Self::HELP,
        Self::LIST,
        Self::SET,
        Self::CREATE,
        Self::DELETE,
        Self::RENAME,
    ];
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 6] = [
        Self::HELP.0,
        Self::LIST.0,
        Self::SET.0,
        Self::CREATE.0,
        Self::DELETE.0,
        Self::RENAME.0,
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
            .map_or("Executing profile command", |(_, desc)| *desc)
    }
}

impl CommandBehavior for ProfileCommand {
    fn requires_acceptance(&self) -> bool {
        // Only certain subcommands require user confirmation
        self.is_modifying_subcommand()
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the subcommand is one we support
        match &self.subcommand {
            Some(_) if self.is_valid_subcommand() => Ok(()),
            Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for profile command", subcmd)),
            None => Err(eyre::eyre!("Subcommand is required for profile command")),
        }
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        let description = match &self.subcommand {
            Some(subcmd) => Self::get_subcommand_description(subcmd).to_string(),
            None => "Executing profile command".to_string(),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    fn format_command(&self) -> String {
        match &self.subcommand {
            Some(subcmd) => format!("/profile {}", subcmd),
            None => "/profile".to_string(),
        }
    }
}
