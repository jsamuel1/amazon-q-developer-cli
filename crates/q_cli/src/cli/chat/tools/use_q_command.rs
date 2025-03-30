use std::collections::HashMap;
use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;
use serde::Deserialize;
use tracing::{
    debug,
    info,
};

use super::command_executor::CommandExecutor;
use super::commands::CommandRegistry;
use super::{
    InvokeOutput,
    OutputKind,
};

/// Request structure for the use_q_command tool
#[derive(Debug, Clone, Deserialize)]
pub struct UseQCommand {
    /// The command to execute (e.g., "quit", "context", "settings")
    pub command: String,

    /// Optional subcommand (e.g., "list", "add", "remove")
    #[serde(default)]
    pub subcommand: Option<String>,

    /// Optional arguments for the command
    #[serde(default)]
    pub args: Option<Vec<String>>,

    /// Optional flags for the command
    #[serde(default)]
    pub flags: Option<HashMap<String, String>>,
}

impl UseQCommand {
    /// Invokes the tool to execute the command
    pub async fn invoke(&self, ctx: &Context, updates: &mut dyn Write) -> Result<InvokeOutput> {
        // Log the command being executed
        info!(
            "Executing Q command: {} {:?} {:?}",
            self.command, self.subcommand, self.args
        );

        // Convert flags HashMap to Vec of tuples for CommandExecutor
        let flags_vec = self
            .flags
            .as_ref()
            .map(|flags| flags.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>());

        // Execute the command using the CommandExecutor
        let result = CommandExecutor::execute_command(
            &self.command,
            self.subcommand.as_deref(),
            self.args.as_deref(),
            flags_vec.as_deref(),
            ctx,
            updates,
        )?;

        debug!("Command execution result: {}", result);

        Ok(InvokeOutput {
            output: OutputKind::Text(result),
        })
    }

    /// Determines if this command requires user acceptance before execution
    pub fn requires_acceptance(&self) -> bool {
        // Get the appropriate command implementation from the registry
        if let Some(command) = CommandRegistry::get_command(&self.command, self.subcommand.as_deref()) {
            command.requires_acceptance()
        } else {
            // If we don't recognize the command, require acceptance to be safe
            true
        }
    }

    /// Formats the command for display or execution
    pub fn format_command(&self) -> String {
        // Get the appropriate command implementation from the registry
        if let Some(command) = CommandRegistry::get_command(&self.command, self.subcommand.as_deref()) {
            command.format_command()
        } else {
            // Fallback formatting if we don't have a specific command implementation
            let mut cmd = String::new();

            // Add slash prefix for slash commands
            let prefix = match self.command.as_str() {
                "quit" | "clear" | "help" => "/",
                _ => "",
            };

            cmd.push_str(&format!("{}{}", prefix, self.command));

            // Add subcommand if present
            if let Some(subcmd) = &self.subcommand {
                cmd.push_str(&format!(" {}", subcmd));
            }

            // Add arguments if present
            if let Some(args) = &self.args {
                for arg in args {
                    cmd.push_str(&format!(" {}", arg));
                }
            }

            // Add flags if present
            if let Some(flags) = &self.flags {
                for (flag, value) in flags {
                    if value.is_empty() {
                        cmd.push_str(&format!(" --{}", flag));
                    } else {
                        cmd.push_str(&format!(" --{}={}", flag, value));
                    }
                }
            }

            cmd
        }
    }

    /// Queue up a description of what this tool will do
    pub fn queue_description(&self, updates: &mut impl Write) -> Result<()> {
        if let Some(command) = CommandRegistry::get_command(&self.command, self.subcommand.as_deref()) {
            command.queue_description(updates)
        } else {
            writeln!(updates, "Executing command: {}", self.format_command())?;
            Ok(())
        }
    }

    /// Validates the command and arguments
    pub fn validate(&self, ctx: &Context) -> Result<()> {
        if let Some(command) = CommandRegistry::get_command(&self.command, self.subcommand.as_deref()) {
            command.validate(ctx)
        } else {
            Err(eyre::eyre!("Unsupported command: {}", self.command))
        }
    }
}
