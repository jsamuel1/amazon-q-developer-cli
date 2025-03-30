use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the help command
pub struct HelpCommand;

impl CommandBehavior for HelpCommand {
    fn requires_acceptance(&self) -> bool {
        // Help command doesn't require user confirmation
        false
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // No validation needed for help command
        Ok(())
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        writeln!(updates, "Showing help information")?;
        Ok(())
    }

    fn format_command(&self) -> String {
        "/help".to_string()
    }

    fn execute(&self, _ctx: &Context, updates: &mut dyn Write) -> Result<String> {
        writeln!(updates, "Showing help information...")?;
        // Note: In a real implementation, we would display the help text
        // For now, we'll just return a message
        Ok("Help information displayed.".to_string())
    }
}
