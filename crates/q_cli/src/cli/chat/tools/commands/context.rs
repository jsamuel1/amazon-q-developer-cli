use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the context command and its subcommands
pub struct ContextCommand {
    subcommand: Option<String>,
}

impl ContextCommand {
    const ADD: &'static str = "add";
    const CLEAR: &'static str = "clear";
    const EXPORT: &'static str = "export";
    // Subcommand constants
    const HELP: &'static str = "help";
    const IMPORT: &'static str = "import";
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 5] =
        [Self::ADD, Self::REMOVE, Self::CLEAR, Self::PRUNE, Self::ROLLBACK];
    const PRUNE: &'static str = "prune";
    const QUERY: &'static str = "query";
    const REMOVE: &'static str = "rm";
    const ROLLBACK: &'static str = "rollback";
    const SHOW: &'static str = "show";
    const SUMMARIZE: &'static str = "summarize";
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 11] = [
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
        let description = match self.subcommand.as_deref() {
            Some(Self::HELP) => "Showing context help information".to_string(),
            Some(Self::SHOW) => "Showing current context".to_string(),
            Some(Self::ADD) => "Adding context from file(s)".to_string(),
            Some(Self::REMOVE) => "Removing context item(s)".to_string(),
            Some(Self::CLEAR) => "Clearing all context".to_string(),
            Some(Self::QUERY) => "Querying conversation history".to_string(),
            Some(Self::PRUNE) => "Pruning conversation history".to_string(),
            Some(Self::ROLLBACK) => "Rolling back conversation to previous point".to_string(),
            Some(Self::SUMMARIZE) => "Summarizing conversation history".to_string(),
            Some(Self::EXPORT) => "Exporting conversation history".to_string(),
            Some(Self::IMPORT) => "Importing conversation history".to_string(),
            Some(subcmd) => format!("Executing context {}", subcmd),
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
}
