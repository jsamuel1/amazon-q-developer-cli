use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the context command and its subcommands
pub struct ContextCommand {
    subcommand: Option<String>,
}

impl ContextCommand {
    const ADD: (&'static str, &'static str) = ("add", "Adding context from file(s)");
    const CLEAR: (&'static str, &'static str) = ("clear", "Clearing all context");
    const EXPORT: (&'static str, &'static str) = ("export", "Exporting conversation history");
    // Subcommand constants with their descriptions
    const HELP: (&'static str, &'static str) = ("help", "Showing context help information");
    const IMPORT: (&'static str, &'static str) = ("import", "Importing conversation history");
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 5] = [
        Self::ADD.0,
        Self::REMOVE.0,
        Self::CLEAR.0,
        Self::PRUNE.0,
        Self::ROLLBACK.0,
    ];
    const PRUNE: (&'static str, &'static str) = ("prune", "Pruning conversation history");
    const QUERY: (&'static str, &'static str) = ("query", "Querying conversation history");
    const REMOVE: (&'static str, &'static str) = ("rm", "Removing context item(s)");
    const ROLLBACK: (&'static str, &'static str) = ("rollback", "Rolling back conversation to previous point");
    const SHOW: (&'static str, &'static str) = ("show", "Showing current context");
    // Map of subcommand names to their descriptions
    const SUBCOMMAND_DESCRIPTIONS: [(&'static str, &'static str); 11] = [
        Self::HELP,
        Self::SHOW,
        Self::ADD,
        Self::REMOVE,
        Self::CLEAR,
        Self::QUERY,
        Self::PRUNE,
        Self::ROLLBACK,
        Self::SUMMARIZE,
        Self::EXPORT,
        Self::IMPORT,
    ];
    const SUMMARIZE: (&'static str, &'static str) = ("summarize", "Summarizing conversation history");
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 11] = [
        Self::HELP.0,
        Self::SHOW.0,
        Self::ADD.0,
        Self::REMOVE.0,
        Self::CLEAR.0,
        Self::QUERY.0,
        Self::PRUNE.0,
        Self::ROLLBACK.0,
        Self::SUMMARIZE.0,
        Self::EXPORT.0,
        Self::IMPORT.0,
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
            .map_or("Executing context command", |(_, desc)| *desc)
    }
}

impl CommandBehavior for ContextCommand {
    fn requires_acceptance(&self) -> bool {
        // Only certain subcommands require user confirmation
        self.is_modifying_subcommand()
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the subcommand is one we support
        match &self.subcommand {
            Some(_) if self.is_valid_subcommand() => Ok(()),
            Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for context command", subcmd)),
            None => Err(eyre::eyre!("Subcommand is required for context command")),
        }
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        let description = match &self.subcommand {
            Some(subcmd) => Self::get_subcommand_description(subcmd).to_string(),
            None => "Executing context command".to_string(),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    fn format_command(&self) -> String {
        match &self.subcommand {
            Some(subcmd) => format!("/context {}", subcmd),
            None => "/context".to_string(),
        }
    }

    fn execute(&self, _ctx: &Context, updates: &mut dyn Write) -> Result<String> {
        let subcmd = self.subcommand.as_deref().unwrap_or("help");
        writeln!(updates, "Executing context {} command...", subcmd)?;

        match subcmd {
            "help" => {
                writeln!(updates, "Available context commands:")?;
                for (cmd, desc) in Self::SUBCOMMAND_DESCRIPTIONS {
                    writeln!(updates, "  {} - {}", cmd, desc)?;
                }
                Ok("Context help information displayed".to_string())
            },
            "show" => {
                // In a real implementation, we would create a context manager and query it
                // For now, we'll just show a sample output
                writeln!(updates, "Current context:")?;
                writeln!(updates, "Profile: default")?;
                writeln!(updates, "Global context:")?;
                writeln!(updates, "  README.md")?;
                writeln!(updates, "Profile context:")?;
                writeln!(updates, "  src/**/*.rs")?;

                Ok("Context information displayed".to_string())
            },
            "add" => {
                // For add, we need path arguments
                // In a real implementation, these would come from the args parameter
                let paths = vec!["example/path.md".to_string()]; // This would come from args

                writeln!(updates, "Adding paths to context: {:?}", paths)?;

                Ok("Context paths added".to_string())
            },
            "rm" => {
                // For remove, we need path arguments
                let paths = vec!["example/path.md".to_string()]; // This would come from args

                writeln!(updates, "Removing paths from context: {:?}", paths)?;

                Ok("Context paths removed".to_string())
            },
            "clear" => {
                writeln!(updates, "Clearing all context paths")?;

                Ok("Context cleared".to_string())
            },
            // Implement other subcommands similarly
            _ => Err(eyre::eyre!("Subcommand '{}' not yet implemented", subcmd)),
        }
    }
}
