use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the clear command
pub struct ClearCommand;

impl CommandBehavior for ClearCommand {
    fn requires_acceptance(&self) -> bool {
        // Clearing conversation history requires user confirmation
        true
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // No validation needed for clear command
        Ok(())
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        writeln!(updates, "Clearing the conversation history")?;
        Ok(())
    }

    fn format_command(&self) -> String {
        "/clear".to_string()
    }
}
