use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;
use tracing::debug;

use crate::cli::chat::tools::commands::CommandRegistry;

/// Executes commands within the q chat system
pub struct CommandExecutor;

impl CommandExecutor {
    /// Execute a command by name with optional subcommand and arguments
    pub fn execute_command(
        command_name: &str,
        subcommand: Option<&str>,
        args: Option<&[String]>,
        flags: Option<&[(String, String)]>,
        ctx: &Context,
        updates: &mut dyn Write,
    ) -> Result<String> {
        debug!(
            "Executing command: {} with subcommand: {:?}, args: {:?}, flags: {:?}",
            command_name, subcommand, args, flags
        );

        // Get the command implementation from the registry
        let command = CommandRegistry::get_command(command_name, subcommand)
            .ok_or_else(|| eyre::eyre!("Unsupported command: {}", command_name))?;

        // Validate the command
        command.validate(ctx)?;

        // Queue up a description of what the command will do
        command.queue_description(updates)?;

        // Execute the command
        let result = command.execute(ctx, updates)?;

        Ok(result)
    }
}
