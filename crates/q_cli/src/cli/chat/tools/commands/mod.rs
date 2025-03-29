use crate::cli::chat::tools::command_behavior::CommandBehavior;

mod clear;
mod context;
mod help;
mod profile;
mod quit;
mod settings;
mod tools;

pub use clear::ClearCommand;
pub use context::ContextCommand;
pub use help::HelpCommand;
pub use profile::ProfileCommand;
pub use quit::QuitCommand;
pub use settings::SettingsCommand;
pub use tools::ToolsCommand;

/// Registry for all available commands that can be executed by the ExecuteQChat tool
pub struct CommandRegistry;

impl CommandRegistry {
    /// Get a command implementation by name
    pub fn get_command(name: &str, subcommand: Option<&str>) -> Option<Box<dyn CommandBehavior>> {
        match name {
            "quit" => Some(Box::new(QuitCommand)),
            "clear" => Some(Box::new(ClearCommand)),
            "help" => Some(Box::new(HelpCommand)),
            "context" => Some(Box::new(ContextCommand::new(subcommand))),
            "profile" => Some(Box::new(ProfileCommand::new(subcommand))),
            "tools" => Some(Box::new(ToolsCommand::new(subcommand))),
            "settings" => Some(Box::new(SettingsCommand::new(subcommand))),
            _ => None,
        }
    }
}
