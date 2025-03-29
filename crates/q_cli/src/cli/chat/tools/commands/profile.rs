use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the profile command and its subcommands
pub struct ProfileCommand {
    subcommand: Option<String>,
}

impl ProfileCommand {
    const CREATE: &'static str = "create";
    const DELETE: &'static str = "delete";
    // Subcommand constants
    const HELP: &'static str = "help";
    const LIST: &'static str = "list";
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 4] = [Self::CREATE, Self::DELETE, Self::RENAME, Self::SET];
    const RENAME: &'static str = "rename";
    const SET: &'static str = "set";
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 6] = [
        Self::HELP,
        Self::LIST,
        Self::SET,
        Self::CREATE,
        Self::DELETE,
        Self::RENAME,
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
        let description = match self.subcommand.as_deref() {
            Some(Self::HELP) => "Showing profile help information".to_string(),
            Some(Self::LIST) => "Listing available profiles".to_string(),
            Some(Self::SET) => "Setting active profile".to_string(),
            Some(Self::CREATE) => "Creating new profile".to_string(),
            Some(Self::DELETE) => "Deleting profile".to_string(),
            Some(Self::RENAME) => "Renaming profile".to_string(),
            Some(subcmd) => format!("Executing profile {}", subcmd),
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
