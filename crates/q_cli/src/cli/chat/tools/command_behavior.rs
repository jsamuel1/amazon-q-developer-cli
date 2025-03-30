use std::io::Write;

use eyre::Result;
use fig_os_shim::Context;

/// Trait defining behavior for commands that can be executed via the use_q_command tool
pub trait CommandBehavior {
    /// Determines if the command requires user acceptance before execution
    fn requires_acceptance(&self) -> bool;

    /// Validates the command parameters
    fn validate(&self, ctx: &Context) -> Result<()>;

    /// Returns a description of what the command will do
    ///
    /// Note: Using a concrete type (Box<dyn Write>) instead of a generic parameter (impl Write)
    /// to ensure the trait is object-safe
    fn queue_description(&self, updates: &mut dyn Write) -> Result<()>;

    /// Formats the command for display
    fn format_command(&self) -> String;

    /// Executes the command
    fn execute(&self, ctx: &Context, updates: &mut dyn Write) -> Result<String>;
}
