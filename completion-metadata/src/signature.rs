use super::{CommandBuilder, Priority, Suggestion};
use crate::{Aliases, Filters, Generators, PathSuggestionType};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

pub struct AnnotatedFlag<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub priority: Priority,
    pub style: FlagStyle,
}

impl<'a> AnnotatedFlag<'a> {
    /// Builds an `AnnotatedFlag` from an option's specific flag name.
    fn from_option(opt: &'a Opt, name: &'a str) -> Self {
        let (style, name) = if let Some(name) = name.strip_prefix("--") {
            (FlagStyle::DoubleDash, name)
        } else {
            (FlagStyle::SingleDash, &name[1..])
        };

        AnnotatedFlag {
            name,
            description: opt.description.as_deref(),
            priority: opt.priority,
            style,
        }
    }
}

/// The prefix style used by a flag. By convention, short-hand flags start with a `-` and
/// long-hand flags start with `--`. However, some programs use a single dash for long-hand
/// flags (such as `java -version`), which is captured by the flag style.
///
/// In the future, this could support Windows-style `/flag` flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagStyle {
    SingleDash,
    DoubleDash,
}

/// Configure how the completion engine will map raw tokens to options/flags in the spec.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParserDirectives {
    /// Flags with long names may begin with just 1 hyphen instead of 2.
    /// https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html
    pub flags_are_posix_noncompliant: bool,

    /// Flags don't need to be spelled out in full, e.g. for `Get-ChildItem` you can provide "-Fi"
    /// instead of "-Filter", but not just "-F" as it might match "-Filter" or "-Force".
    pub flags_match_unique_prefix: bool,

    /// This command is case-insensitive _even if_ the user's filesystem is case-sensitive.
    pub always_case_insensitive: bool,
}

/// A `Signature` defines a command or a subcommand.
/// `Signature`s are recursive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub alias_generator: Option<AliasGeneratorName>,
    pub description: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub subcommands: Option<Vec<Signature>>,
    pub options: Option<Vec<Opt>>,
    pub priority: Priority,
    pub parser_directives: ParserDirectives,
}

impl Signature {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[Argument] {
        self.arguments.as_deref().unwrap_or(&[])
    }

    pub fn subcommands(&self) -> &[Signature] {
        self.subcommands.as_deref().unwrap_or(&[])
    }

    pub fn options(&self) -> &[Opt] {
        self.options.as_deref().unwrap_or(&[])
    }
}

/// Heuristic to determine if a flag name is a short-hand flag or not.
///
/// * A single dash followed by a single character (`-h`, `-v`, etc.) is short-hand, unless the second character is also a dash.
/// * A single dash followed by multiple characters (`-version`) is long-hand
/// * Two dashes followed by 0 or more characters is long-hand
fn is_short_hand_flag(name: &str) -> bool {
    name.len() == 2 && name.starts_with('-') && name != "--"
}

/// Tests if `name` is a long-hand flag name. A long-hand flag is a string
/// starting with `-` that is not a short-hand flag.
fn is_long_hand_flag(name: &str) -> bool {
    name.starts_with('-') && !is_short_hand_flag(name)
}

impl Signature {
    /// Returns a list of the short-hand flags.
    // TODO(alokedesai): Investigate why these are stored in `Vec` instead of precomputed.
    pub fn short_hand_flags(&self) -> impl Iterator<Item = AnnotatedFlag> + '_ {
        self.options
            .iter()
            .flat_map(|options| options.iter())
            .flat_map(|option| {
                option
                    .exact_string
                    .iter()
                    .filter(|&name| is_short_hand_flag(name))
                    .map(|name| AnnotatedFlag::from_option(option, name))
            })
    }

    /// Returns a list of long-hand flags.
    // TODO(alokedesai): Investigate why these are stored in `Vec` instead of precomputed.
    pub fn long_hand_flags(&self) -> impl Iterator<Item = AnnotatedFlag> + '_ {
        self.options
            .iter()
            .flat_map(|options| options.iter())
            .flat_map(|option| {
                option
                    .exact_string
                    .iter()
                    .filter(|&name| is_long_hand_flag(name))
                    .map(|name| AnnotatedFlag::from_option(option, name))
            })
    }

    pub fn alias<'a>(&self, aliases: Option<&'a Aliases>) -> Option<&'a Alias> {
        self.alias_generator.as_ref().and_then(|alias_name| {
            let aliases = match aliases {
                None => {
                    log::error!(
                        "Signature {:?} specified alias {:?} but none are specified",
                        &self.name,
                        alias_name
                    );
                    return None;
                }
                Some(aliases) => aliases,
            };

            match aliases.get(alias_name) {
                None => {
                    log::error!(
                        "Signature {:?} specified alias {:?} but it wasn't specified",
                        &self.name,
                        alias_name
                    );
                    None
                }
                Some(alias) => Some(alias),
            }
        })
    }
}

/// An `Opt` is an option. It adds information to a command.
/// We use the shortname `Opt` here to avoid the conflict with std::Option
/// It takes the forms of --name, -name, --name=arg, -name=arg, --name arg, -name arg.
/// The ones that do not take an argument are called flags and are boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Opt {
    // `--name`, `-n`, etc.
    pub exact_string: Vec<String>,
    pub description: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub required: bool,
    pub priority: Priority,
}

impl Opt {
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.exact_string.iter().map(AsRef::as_ref)
    }

    pub fn arguments(&self) -> &[Argument] {
        self.arguments.as_deref().unwrap_or_default()
    }

    pub fn is_switch(&self) -> bool {
        match &self.arguments {
            Some(args) => args.is_empty(),
            None => true,
        }
    }

    pub fn has_name(&self, name: &str) -> bool {
        self.exact_string.iter().any(|s| s.as_str() == name)
    }
}

/// An `Argument` indicates when a Signature or an Opt takes a value as a parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    pub display_name: Option<String>,
    pub description: Option<String>,
    // If the argument can be repeated a variable number of times (such as git add [file name...])
    pub is_variadic: bool,
    // `argument_types` is a vector because an argument can derive suggestions from multiple places.
    // If empty, the  parser will accept any string.
    pub argument_types: Vec<ArgumentType>,
    pub optional: IsArgumentOptional,
    /// Whether this argument should itself be a top-level command (such as `sudo <arg>` or `time <arg>`). If true,
    /// the completer will surface completions for top level command here.
    pub is_command: bool,
    /// Whether to skip generator validation for this argument. If false, this arg's generators
    /// will be used to validate args in command suggestions.
    /// In general, this should be true if the generators are not exhaustive of all valid arguments.
    pub skip_generator_validation: bool,
}

impl Argument {
    pub fn name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn is_variadic(&self) -> bool {
        self.is_variadic
    }

    pub fn is_required(&self) -> bool {
        matches!(self.optional, IsArgumentOptional::Required)
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn generator_by_name<'a>(
        &self,
        generators: Option<&'a Generators>,
        generator_name: &GeneratorName,
    ) -> Option<&'a Generator> {
        let generators = match generators {
            None => {
                log::error!(
                    "Argument {:?} specified generator {:?} but none are specified",
                    &self.display_name,
                    generator_name
                );
                return None;
            }
            Some(generators) => generators,
        };

        match generators.get(generator_name) {
            None => {
                log::error!(
                    "Argument {:?} specified generator {:?} but it wasn't specified",
                    &self.display_name,
                    generator_name
                );
                None
            }
            Some(generator) => Some(generator),
        }
    }

    pub fn filter_template_by_name<'a>(
        &self,
        filters: Option<&'a Filters>,
        filter_template_name: &FilterTemplateSuggestion,
    ) -> Option<&'a TemplateFilter> {
        let filters = match filters {
            None => {
                log::error!(
                    "Argument {:?} specified filter {:?} but none are specified",
                    &self.display_name,
                    filter_template_name
                );
                return None;
            }
            Some(filters) => filters,
        };

        match filters.get(filter_template_name) {
            None => {
                log::error!(
                    "Argument {:?} specified filter {:?} but it wasn't specified",
                    &self.display_name,
                    filter_template_name
                );
                None
            }
            Some(filter) => Some(filter),
        }
    }
}

type DefaultValue = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IsArgumentOptional {
    Optional(Option<DefaultValue>),
    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArgumentType {
    Suggestion(Suggestion),
    Template(Template),
    Generator(GeneratorName),
    Alias(AliasGeneratorName),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GeneratorName(pub String);

impl GeneratorName {
    pub fn new(str: impl Into<String>) -> Self {
        GeneratorName(str.into())
    }
}

impl From<&'static str> for GeneratorName {
    fn from(str: &'static str) -> Self {
        Self(str.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilterTemplateSuggestion(pub String);

impl From<&'static str> for FilterTemplateSuggestion {
    fn from(str: &'static str) -> Self {
        Self(str.into())
    }
}

/// A Generator returns GeneratorResults. If is_ordered is true, then Warp
/// will not reorder the suggestions. Otherwise, Warp will provide a default
/// lexicographical sort over the suggestions.
/// Note: any Suggestion in suggestions with priority other than Default
/// will be treated separately and not sorted with the other suggestions.
#[derive(Debug, PartialEq, Eq)]
pub struct GeneratorResults {
    pub suggestions: Vec<Suggestion>,
    pub is_ordered: bool,
}

impl Default for GeneratorResults {
    fn default() -> Self {
        Self {
            suggestions: vec![],
            is_ordered: true, // vacuously true
        }
    }
}

/// Helper trait to transform an iterator over Suggestions into GeneratorResults.
pub trait GeneratorResultsCollector: Iterator<Item = Suggestion> {
    fn collect_ordered_results(self) -> GeneratorResults
    where
        Self: Sized,
    {
        GeneratorResults {
            suggestions: self.collect(),
            is_ordered: true,
        }
    }

    fn collect_unordered_results(self) -> GeneratorResults
    where
        Self: Sized,
    {
        GeneratorResults {
            suggestions: self.collect(),
            is_ordered: false,
        }
    }
}

#[derive(Clone)]
pub enum GeneratorProcess {
    /// Tokens should contain every token in the input, including the last one which may still be incomplete.
    /// The second bool argument is whether there is trailing whitespace in the command. This is
    /// necessary so the completions generator can tell whether it's completing a partial token or a new token.
    /// The third argument is a list of environment variables, passed as ["KEY=VALUE", ...].
    /// Note that some options can take multiple whitespace-delimited args, so it's up to the generator to actually determine
    /// what suggestions to provide for a new token.
    CommandFromTokens(fn(&[&str], bool, &[String]) -> CommandBuilder),
    ShellCommand(CommandBuilder),
}

impl Debug for GeneratorProcess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandFromTokens(_) => write!(f, "Context Generator"),
            Self::ShellCommand(s) => write!(f, "{:?}", s),
        }
    }
}

impl<T> GeneratorResultsCollector for T where T: Iterator<Item = Suggestion> {}

/// A `Generator` runs a shell command and performs an action on the output to provide GeneratorResults.
#[derive(Clone)]
pub struct Generator {
    pub process: GeneratorProcess,
    // For now, `on_complete` only processes stdout.
    pub on_complete_callback: fn(&str) -> GeneratorResults,
}

impl Generator {
    pub fn script(
        shell_command: impl Into<CommandBuilder>,
        on_complete_callback: fn(&str) -> GeneratorResults,
    ) -> Self {
        Generator {
            process: GeneratorProcess::ShellCommand(shell_command.into()),
            on_complete_callback,
        }
    }

    pub fn command_from_tokens(
        command_from_tokens: fn(&[&str], bool, &[String]) -> CommandBuilder,
        on_complete_callback: fn(&str) -> GeneratorResults,
    ) -> Self {
        Generator {
            process: GeneratorProcess::CommandFromTokens(command_from_tokens),
            on_complete_callback,
        }
    }
}

impl Generator {
    pub fn on_complete(&self, input: &str) -> GeneratorResults {
        (self.on_complete_callback)(input)
    }
}

impl Debug for Generator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.process)
    }
}

/// Prebuilt `Generator`s
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemplateType {
    Files {
        must_exist: bool,
    },
    Folders {
        must_exist: bool,
    },
    #[allow(dead_code)]
    FilesAndFolders,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    // The type of the prebuilt generator template.
    pub type_name: TemplateType,
    // Name of the filter. This is used to reference the filtering function.
    pub filter_name: Option<FilterTemplateSuggestion>,
}

/// A template filter function. This takes in a generated Suggestion and returned
/// a modified suggestion or None if the suggestion is filtered out.
#[derive(Clone)]
pub struct TemplateFilter(pub fn(Suggestion, PathSuggestionType) -> Option<Suggestion>);

impl TemplateFilter {
    pub fn filter(&self, input: Suggestion, type_name: PathSuggestionType) -> Option<Suggestion> {
        (self.0)(input, type_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AliasGeneratorName(pub String);

impl From<&str> for AliasGeneratorName {
    fn from(str: &str) -> Self {
        Self(str.into())
    }
}

impl fmt::Display for AliasGeneratorName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An `Alias` takes in a shell command and expands it if there is any command specific aliases.
#[derive(Clone)]
pub struct Alias {
    // Given a list of command tokens, return the shell command that will generate aliases.
    pub command_from_tokens: fn(&[&str]) -> String,
    // Given the command from `command_from_tokens`, the list of tokens and the index of the current token, return the expanded command.
    pub on_complete_callback: fn(&str, &[&str], usize) -> Option<String>,
}

impl Alias {
    pub fn new(
        command_from_tokens: fn(&[&str]) -> String,
        on_complete_callback: fn(&str, &[&str], usize) -> Option<String>,
    ) -> Self {
        Self {
            command_from_tokens,
            on_complete_callback,
        }
    }

    pub fn command(&self, input: &[&str]) -> String {
        (self.command_from_tokens)(input)
    }

    pub fn on_complete(
        &self,
        alias_command_output: &str,
        tokens: &[&str],
        token_idx: usize,
    ) -> Option<String> {
        (self.on_complete_callback)(alias_command_output, tokens, token_idx)
    }
}
