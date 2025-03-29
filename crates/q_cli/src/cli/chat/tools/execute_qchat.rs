use std::collections::HashMap;
use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;
use serde::Deserialize;
use tracing::{
    debug,
    info,
};

use super::command_behavior::CommandBehavior;
use super::{
    InvokeOutput,
    OutputKind,
};

/// Request structure for the execute_qchat tool
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteQChat {
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

impl CommandBehavior for ExecuteQChat {
    /// Determines if this command requires user acceptance before execution
    fn requires_acceptance(&self) -> bool {
        // Commands that modify state or exit the application require acceptance
        match self.command.as_str() {
            "quit" => true,
            "clear" => true,
            "context" => {
                matches!(
                    self.subcommand.as_deref(),
                    Some("add" | "rm" | "clear" | "prune" | "rollback")
                )
            },
            "profile" => {
                matches!(self.subcommand.as_deref(), Some("create" | "delete" | "rename" | "set"))
            },
            "tools" => {
                matches!(
                    self.subcommand.as_deref(),
                    Some("enable" | "disable" | "install" | "uninstall" | "update")
                )
            },
            "settings" => {
                matches!(self.subcommand.as_deref(), Some("set" | "reset"))
            },
            _ => false,
        }
    }

    /// Validates the command and arguments
    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the command is one we support
        match self.command.as_str() {
            "quit" | "clear" | "help" => Ok(()),
            "context" => {
                // Validate subcommand for context
                match self.subcommand.as_deref() {
                    Some(
                        "help" | "show" | "add" | "rm" | "clear" | "query" | "prune" | "rollback" | "summarize"
                        | "export" | "import",
                    ) => Ok(()),
                    Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for context command", subcmd)),
                    None => Err(eyre::eyre!("Subcommand is required for context command")),
                }
            },
            "profile" => {
                // Validate subcommand for profile
                match self.subcommand.as_deref() {
                    Some("help" | "list" | "set" | "create" | "delete" | "rename") => Ok(()),
                    Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for profile command", subcmd)),
                    None => Err(eyre::eyre!("Subcommand is required for profile command")),
                }
            },
            "tools" => {
                // Validate subcommand for tools
                match self.subcommand.as_deref() {
                    Some("list" | "enable" | "disable" | "install" | "uninstall" | "update" | "info") => Ok(()),
                    Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for tools command", subcmd)),
                    None => Err(eyre::eyre!("Subcommand is required for tools command")),
                }
            },
            "settings" => {
                // Validate subcommand for settings
                match self.subcommand.as_deref() {
                    Some("list" | "set" | "reset") => Ok(()),
                    Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for settings command", subcmd)),
                    None => Err(eyre::eyre!("Subcommand is required for settings command")),
                }
            },
            cmd => Err(eyre::eyre!("Unsupported command: {}", cmd)),
        }
    }

    /// Queues up a description of what this tool will do
    fn queue_description(&self, updates: &mut impl Write) -> Result<()> {
        let description = match self.command.as_str() {
            "quit" => "Exiting the application".to_string(),
            "clear" => "Clearing the conversation history".to_string(),
            "help" => "Showing help information".to_string(),
            "context" => match self.subcommand.as_deref() {
                Some("help") => "Showing context help information".to_string(),
                Some("show") => "Showing current context".to_string(),
                Some("add") => format!(
                    "Adding context from {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"file".to_string())
                ),
                Some("rm") => format!(
                    "Removing context {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"item".to_string())
                ),
                Some("clear") => "Clearing all context".to_string(),
                Some("query") => "Querying conversation history".to_string(),
                Some("prune") => "Pruning conversation history".to_string(),
                Some("rollback") => "Rolling back conversation to previous point".to_string(),
                Some("summarize") => "Summarizing conversation history".to_string(),
                Some("export") => "Exporting conversation history".to_string(),
                Some("import") => "Importing conversation history".to_string(),
                Some(subcmd) => format!("Executing context {}", subcmd),
                None => "Executing context command".to_string(),
            },
            "profile" => match self.subcommand.as_deref() {
                Some("help") => "Showing profile help information".to_string(),
                Some("list") => "Listing available profiles".to_string(),
                Some("set") => format!(
                    "Setting profile to {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"profile".to_string())
                ),
                Some("create") => format!(
                    "Creating new profile {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"profile".to_string())
                ),
                Some("delete") => format!(
                    "Deleting profile {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"profile".to_string())
                ),
                Some("rename") => "Renaming profile".to_string(),
                Some(subcmd) => format!("Executing profile {}", subcmd),
                None => "Executing profile command".to_string(),
            },
            "tools" => match self.subcommand.as_deref() {
                Some("list") => "Listing available tools".to_string(),
                Some("enable") => format!(
                    "Enabling tool {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"tool".to_string())
                ),
                Some("disable") => format!(
                    "Disabling tool {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"tool".to_string())
                ),
                Some("install") => format!(
                    "Installing tool {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"tool".to_string())
                ),
                Some("uninstall") => format!(
                    "Uninstalling tool {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"tool".to_string())
                ),
                Some("update") => "Updating tools".to_string(),
                Some("info") => format!(
                    "Showing information about tool {}",
                    self.args
                        .as_ref()
                        .and_then(|a| a.first())
                        .unwrap_or(&"tool".to_string())
                ),
                Some(subcmd) => format!("Executing tools {}", subcmd),
                None => "Executing tools command".to_string(),
            },
            "settings" => match self.subcommand.as_deref() {
                Some("list") => "Listing current settings".to_string(),
                Some("set") => {
                    if let (Some(args), Some(first_arg)) =
                        (self.args.as_ref(), self.args.as_ref().and_then(|a| a.first()))
                    {
                        if args.len() > 1 {
                            format!(
                                "Setting {} to {}",
                                first_arg,
                                args.get(1).unwrap_or(&"value".to_string())
                            )
                        } else {
                            format!("Setting {}", first_arg)
                        }
                    } else {
                        "Setting configuration".to_string()
                    }
                },
                Some("reset") => "Resetting settings to default".to_string(),
                Some(subcmd) => format!("Executing settings {}", subcmd),
                None => "Executing settings command".to_string(),
            },
            cmd => format!("Executing command: {}", cmd),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    /// Formats the command for display or execution
    fn format_command(&self) -> String {
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

impl ExecuteQChat {
    /// Invokes the tool to execute the command
    pub async fn invoke(&self, _ctx: &Context, _updates: &mut impl Write) -> Result<InvokeOutput> {
        // Log the command being executed
        info!(
            "Executing qchat command: {} {:?} {:?}",
            self.command, self.subcommand, self.args
        );

        // Format the command for execution
        let formatted_command = self.format_command();

        // In a real implementation, we would execute the command here
        // For now, we'll just return a message about what would be executed
        let output = format!("Command executed: {}", formatted_command);

        // TODO: Implement actual command execution by integrating with the existing command infrastructure
        // This would involve calling into the appropriate command handlers in the q_cli crate

        debug!("Command execution result: {}", output);

        Ok(InvokeOutput {
            output: OutputKind::Text(output),
        })
    }
}
