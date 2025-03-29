//! Configuration for command execution in the chat module.

/// Configuration for command execution
///
/// This struct holds preferences and settings related to command execution
/// that are not part of the conversation state.
#[derive(Debug, Clone, Default)]
pub struct CommandExecutionConfig {
    /// Whether to accept all tool invocations without prompting
    pub accept_all: bool,
}

impl CommandExecutionConfig {
    /// Toggle the accept_all flag and return the new value
    pub fn toggle_accept_all(&mut self) -> bool {
        self.accept_all = !self.accept_all;
        self.accept_all
    }

    /// Get the current value of the accept_all flag
    pub fn get_accept_all(&self) -> bool {
        self.accept_all
    }
}
