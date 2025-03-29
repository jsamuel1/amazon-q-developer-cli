use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the quit command
pub struct QuitCommand;

impl CommandBehavior for QuitCommand {
    fn requires_acceptance(&self) -> bool {
        // Quitting always requires user confirmation
        true
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // No validation needed for quit command
        Ok(())
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        writeln!(updates, "Exiting the application")?;
        Ok(())
    }

    fn format_command(&self) -> String {
        "/quit".to_string()
    }
}
