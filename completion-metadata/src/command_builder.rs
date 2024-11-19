//! Module containing the definition of a [`CommandBuilder`], which is used to create commands for a
//! [`crate::Generator`] that is agnostic of the shell that the command would be running in.

use crate::Shell;
use std::borrow::Cow;

#[derive(Clone, Debug)]
enum CommandPart {
    /// A single command/
    SingleCommand(String),
    /// Two commands separated by an and (i.e. `A && B``)
    And(Box<CommandPart>, Box<CommandPart>),
    /// Two command separated by a pipe (i.e. `A | B`)
    Pipe(Box<CommandPart>, Box<CommandPart>),
}

impl CommandPart {
    fn command(&self, shell: Shell) -> Cow<str> {
        match self {
            CommandPart::SingleCommand(command) => command.into(),
            CommandPart::And(command_1, command_2) => format!(
                "{} && {}",
                command_1.command(shell),
                command_2.command(shell)
            )
            .into(),
            CommandPart::Pipe(command_1, command_2) => format!(
                "{} {} | {}",
                command_1.command(shell),
                shell.ignore_stderr(),
                command_2.command(shell)
            )
            .into(),
        }
    }
}

/// A builder to generate commands to be run a session in a way that is agnostic of the shell it is
/// running within.
#[derive(Clone, Debug)]
pub struct CommandBuilder(CommandPart);

// TOOD(alokedesai): Remove these temporary conversion methods once all of the generators in this
// crate have been migrated to the new struct.
impl From<String> for CommandBuilder {
    fn from(command: String) -> Self {
        Self::single_command(command)
    }
}

impl From<&str> for CommandBuilder {
    fn from(command: &str) -> Self {
        Self::single_command(command.to_owned())
    }
}

impl CommandBuilder {
    /// Constructs a new [`CommandBuilder`] for a _single_ command.
    /// See the [`Self::and`] and [`Self::pipe`] constructors for chaining multiple commands
    /// together.
    pub fn single_command(command: impl Into<String>) -> Self {
        Self(CommandPart::SingleCommand(command.into()))
    }

    /// Constructs a new [`CommandBuilder`] for a series of commands that should be and'd together
    /// (i.e. `second_command` should only run iff `first_command` succeeds).
    pub fn and(first_command: impl Into<String>, second_command: impl Into<String>) -> Self {
        Self(CommandPart::And(
            Box::new(CommandPart::SingleCommand(first_command.into())),
            Box::new(CommandPart::SingleCommand(second_command.into())),
        ))
    }

    /// Constructs a new [`CommandBuilder`] for a series of commands that should be piped together.
    /// Concretely, this means the stdout of `first_command` is passed as input to `second_command`.
    /// NOTE any stderr output from `first_command` is ignored.
    pub fn pipe(first_command: impl Into<String>, second_command: impl Into<String>) -> Self {
        Self(CommandPart::Pipe(
            Box::new(CommandPart::SingleCommand(first_command.into())),
            Box::new(CommandPart::SingleCommand(second_command.into())),
        ))
    }

    /// Returns the constructed command given the current shell type.
    pub fn build(&self, shell: Shell) -> Cow<str> {
        self.0.command(shell)
    }
}
