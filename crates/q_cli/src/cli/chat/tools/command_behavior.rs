use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

/// Trait defining behavior for commands that can be executed via the ExecuteQChat tool
pub trait CommandBehavior {
    /// Determines if the command requires user acceptance before execution
    fn requires_acceptance(&self) -> bool;

    /// Validates the command parameters
    fn validate(&self, ctx: &Context) -> Result<()>;

    /// Returns a description of what the command will do
    fn queue_description(&self, updates: &mut impl Write) -> Result<()>;

    /// Formats the command for display
    fn format_command(&self) -> String;
}
