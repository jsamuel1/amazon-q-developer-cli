# Amazon Q Development Guidelines

Always follow these guidelines when assisting in development for the Amazon Q CLI.

## AmazonQ.md

DO NOT create or modify an AmazonQ.md file unless I explicitly tell you to do so.

## Rust Best Practices

### File Operations

When working with file operations in Rust:

1. Prefer using the simpler `fs::read_to_string()` and `fs::write()` functions over verbose `File::open()` + `read_to_string()` or `File::create()` + `write_all()` combinations
2. Avoid the `#[allow(clippy::verbose_file_reads)]` annotation by using the recommended methods
3. Use `serde_json::to_string_pretty()` + `fs::write()` instead of creating a file and then writing to it with `serde_json::to_writer_pretty()`
4. Keep imports organized by functionality (e.g., group path-related imports together)

### Code Organization and Design Patterns

1. Prefer trait-based polymorphism over large match statements on enums
   - Use traits to define behavior interfaces
   - Implement traits for different types rather than using match statements on enum variants
   - This improves extensibility, maintainability, and testability

2. Follow the Command pattern for implementing commands
   - Define a common interface (trait) for all commands
   - Each command should be its own type implementing the common interface
   - Use a registry or factory to look up command implementations by name
   - Avoid large match statements that need to be updated for each new command

3. Use dependency injection where appropriate
   - Pass dependencies as parameters rather than creating them inside functions
   - This improves testability and flexibility

4. Separate behavior from data
   - Define data structures separately from the code that operates on them
   - Use methods or free functions to implement behavior on data structures

5. Use appropriate parameter types for trait objects
   - Use `&dyn Trait` for read-only access to trait objects
   - Use `&mut dyn Trait` when the trait object needs to be modified
   - Use `Box<dyn Trait>` when ownership needs to be transferred
   - Consider using `Arc<dyn Trait>` for shared ownership in multi-threaded contexts
   - Prefer static dispatch with generics (`impl Trait`) over dynamic dispatch (`dyn Trait`) when performance is critical

### q_cli Crate Specific Patterns

1. Registry Pattern for Command Lookup
   - Use a registry pattern (like `CommandRegistry`) for command lookup and discovery
   - Centralize command registration to make adding new commands easier
   - Avoid direct dependencies between command implementations

2. Tool-based Architecture
   - Implement capabilities as tools with consistent interfaces but specialized behavior
   - Use a `ToolManager` to dynamically load and manage tools
   - Separate tool definition from tool execution

3. Interface Segregation
   - Define focused interfaces (like `CommandBehavior`) for specific behaviors
   - Follow the Interface Segregation Principle from SOLID
   - Make components more modular and easier to test

4. Configuration Management
   - Load and merge configurations from multiple sources (global and local)
   - Prioritize local configurations over global ones with appropriate warnings
   - Use consistent serialization/deserialization patterns

5. Error Handling Patterns
   - Use the `eyre` crate consistently for error handling
   - Add context to errors using `.context()` or `.wrap_err()`
   - Make error messages user-friendly and actionable

6. Asynchronous Programming Patterns
   - Use async/await for I/O operations
   - Handle concurrent operations with `buffer_unordered`
   - Process streams for handling multiple asynchronous tasks

7. Path Handling and Sanitization
   - Implement proper path handling, including tilde expansion and sanitization
   - Consider relative vs. absolute path handling for better user experience
   - Ensure cross-platform path handling

8. Testing Strategies
   - Write unit tests with mocked contexts
   - Create test utilities for setting up test environments
   - Use parameterized tests to cover multiple scenarios

9. Telemetry and Logging
   - Implement structured logging with different verbosity levels
   - Track command usage with telemetry
   - Configure log destinations (file vs stdout) based on context

## Git

### Committing Changes

Follow the git best practice of committing early and often. Run `git commit` often, but DO NOT ever run `git push`

BEFORE committing a change, ALWAYS do the following steps:

1. Run `cargo build` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
2. Run `cargo test` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
3. Run `cargo +nightly fmt` to auto-format the code
4. Commit the changes

### Commit Messages

All commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification and include best practices:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

🤖 Assisted by [Amazon Q Developer](https://aws.amazon.com/q/developer)
```

Types:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools
- ci: Changes to CI configuration files and scripts

Best practices:
- Use the imperative mood ("add" not "added" or "adds")
- Don't end the subject line with a period
- Limit the subject line to 50 characters
- Capitalize the subject line
- Separate subject from body with a blank line
- Use the body to explain what and why vs. how
- Wrap the body at 72 characters

Example:
```
feat(lambda): Add Go implementation of DDB stream forwarder

Replace Node.js Lambda function with Go implementation to reduce cold
start times. The new implementation supports forwarding to multiple SQS
queues and maintains the same functionality as the original.

🤖 Assisted by [Amazon Q Developer](https://aws.amazon.com/q/developer)
```
