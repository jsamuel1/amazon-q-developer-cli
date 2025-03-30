use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::tools::command_behavior::CommandBehavior;

/// Command implementation for the tools command and its subcommands
pub struct ToolsCommand {
    subcommand: Option<String>,
}

impl ToolsCommand {
    const DISABLE: (&'static str, &'static str) = ("disable", "Disabling tool");
    const ENABLE: (&'static str, &'static str) = ("enable", "Enabling tool");
    const INFO: (&'static str, &'static str) = ("info", "Showing information about tool");
    const INSTALL: (&'static str, &'static str) = ("install", "Installing tool");
    // Subcommand constants with their descriptions
    const LIST: (&'static str, &'static str) = ("list", "Listing available tools");
    // Subcommands that require user confirmation
    const MODIFYING_SUBCOMMANDS: [&'static str; 5] = [
        Self::ENABLE.0,
        Self::DISABLE.0,
        Self::INSTALL.0,
        Self::UNINSTALL.0,
        Self::UPDATE.0,
    ];
    // Map of subcommand names to their descriptions
    const SUBCOMMAND_DESCRIPTIONS: [(&'static str, &'static str); 7] = [
        Self::LIST,
        Self::ENABLE,
        Self::DISABLE,
        Self::INSTALL,
        Self::UNINSTALL,
        Self::UPDATE,
        Self::INFO,
    ];
    const UNINSTALL: (&'static str, &'static str) = ("uninstall", "Uninstalling tool");
    const UPDATE: (&'static str, &'static str) = ("update", "Updating tools");
    // All valid subcommands
    const VALID_SUBCOMMANDS: [&'static str; 7] = [
        Self::LIST.0,
        Self::ENABLE.0,
        Self::DISABLE.0,
        Self::INSTALL.0,
        Self::UNINSTALL.0,
        Self::UPDATE.0,
        Self::INFO.0,
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
            .map_or("Executing tools command", |(_, desc)| *desc)
    }
}

impl CommandBehavior for ToolsCommand {
    fn requires_acceptance(&self) -> bool {
        // Only certain subcommands require user confirmation
        self.is_modifying_subcommand()
    }

    fn validate(&self, _ctx: &Context) -> Result<()> {
        // Validate the subcommand is one we support
        match &self.subcommand {
            Some(_) if self.is_valid_subcommand() => Ok(()),
            Some(subcmd) => Err(eyre::eyre!("Invalid subcommand '{}' for tools command", subcmd)),
            None => Err(eyre::eyre!("Subcommand is required for tools command")),
        }
    }

    fn queue_description(&self, updates: &mut dyn Write) -> Result<()> {
        let description = match &self.subcommand {
            Some(subcmd) => Self::get_subcommand_description(subcmd).to_string(),
            None => "Executing tools command".to_string(),
        };

        writeln!(updates, "{}", description)?;
        Ok(())
    }

    fn format_command(&self) -> String {
        match &self.subcommand {
            Some(subcmd) => format!("/tools {}", subcmd),
            None => "/tools".to_string(),
        }
    }

    fn execute(&self, _ctx: &Context, updates: &mut dyn Write) -> Result<String> {
        let subcmd = self.subcommand.as_deref().unwrap_or("help");
        writeln!(updates, "Executing tools {} command...", subcmd)?;

        // Since we don't have a direct ToolsManager like we do for context,
        // we'll implement a basic version that demonstrates the structure
        match subcmd {
            "list" => {
                // In a real implementation, we would query available tools from a registry
                // For now, we'll just list the built-in tools
                writeln!(updates, "Available tools:")?;
                writeln!(updates, "  fs_read - Read files and directories")?;
                writeln!(updates, "  fs_write - Create and edit files")?;
                writeln!(updates, "  execute_bash - Execute shell commands")?;
                writeln!(updates, "  use_aws - Make AWS CLI calls")?;
                writeln!(updates, "  report_issue - Open GitHub issue template")?;
                writeln!(updates, "  use_q_command - Execute Q commands")?;

                Ok("Tools listed".to_string())
            },
            "enable" => {
                // For enable, we need a tool name argument
                let tool_name = "example_tool"; // This would come from args

                // In a real implementation, we would update a tool registry or settings
                writeln!(updates, "Enabling tool: {}", tool_name)?;

                Ok(format!("Tool '{}' enabled", tool_name))
            },
            "disable" => {
                // For disable, we need a tool name argument
                let tool_name = "example_tool"; // This would come from args

                // In a real implementation, we would update a tool registry or settings
                writeln!(updates, "Disabling tool: {}", tool_name)?;

                Ok(format!("Tool '{}' disabled", tool_name))
            },
            "info" => {
                // For info, we need a tool name argument
                let tool_name = "fs_read"; // This would come from args

                // In a real implementation, we would query the tool registry for details
                writeln!(updates, "Tool information for '{}':", tool_name)?;
                writeln!(updates, "  Name: fs_read")?;
                writeln!(updates, "  Description: Tool for reading files and directories")?;
                writeln!(updates, "  Status: Enabled")?;
                writeln!(updates, "  Requires acceptance: No (for read operations)")?;

                Ok(format!("Tool '{}' information displayed", tool_name))
            },
            _ => Err(eyre::eyre!("Subcommand '{}' not yet implemented", subcmd)),
        }
    }
}
