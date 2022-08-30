use super::{Priority, Suggestion};
use crate::{Aliases, Filters, Generators, PathSuggestionType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

pub struct AnnotatedFlag<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub priority: Priority,
}

/// A `Signature` defines a command or a subcommand.
/// `Signature`s are recursive.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Signature {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub subcommands: Option<Vec<Signature>>,
    pub options: Option<Vec<Opt>>,
    pub priority: Priority,
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

impl Signature {
    /// Returns a list of the short-hand flags.
    // TODO(alokedesai): Investigate why these are stored in `Vec` instead of precomputed.
    pub fn short_hand_flags(&self) -> impl Iterator<Item = AnnotatedFlag> + '_ {
        self.options
            .iter()
            .flat_map(|options| options.iter())
            .flat_map(|option| {
                option.exact_string.iter().filter_map(|name| {
                    (name.starts_with('-') && !name.starts_with("--")).then(|| AnnotatedFlag {
                        name: &name[1..],
                        description: option.description.as_deref(),
                        priority: option.priority,
                    })
                })
            })
    }

    /// Returns a list of long-hand flags.
    // TODO(alokedesai): Investigate why these are stored in `Vec` instead of precomputed.
    pub fn long_hand_flags(&self) -> impl Iterator<Item = AnnotatedFlag> + '_ {
        self.options
            .iter()
            .flat_map(|options| options.iter())
            .flat_map(|option| {
                option.exact_string.iter().filter_map(|name| {
                    (name.starts_with("--")).then(|| AnnotatedFlag {
                        name: &name[2..],
                        description: option.description.as_deref(),
                        priority: option.priority,
                    })
                })
            })
    }
}

/// An `Opt` is an option. It adds information to a command.
/// We use the shortname `Opt` here to avoid the conflict with std::Option
/// It takes the forms of --name, -name, --name=arg, -name=arg, --name arg, -name arg.
/// The ones that do not take an argument are called flags and are boolean.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Opt {
    // `--name`, `-n`, etc.
    pub exact_string: Vec<String>,
    pub description: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub required: bool,
    pub priority: Priority,
}

impl Opt {
    pub fn names(&self) -> &[String] {
        self.exact_string.as_slice()
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

    #[allow(dead_code)]
    pub fn required(&self) -> bool {
        self.required
    }
}

impl Opt {
    pub fn get_short_hand_flags(&self) -> Vec<String> {
        self.exact_string
            .iter()
            .filter(|s| s.starts_with('-') && !s.starts_with("--"))
            .map(|s| s[1..].to_string())
            .collect()
    }

    pub fn get_long_hand_flags(&self) -> Vec<String> {
        self.exact_string
            .iter()
            .filter(|s| s.starts_with("--"))
            .map(|s| s[2..].to_string())
            .collect()
    }

    // Whether the option has the given name.
    pub fn has_name(&self, name: &str) -> bool {
        self.exact_string.iter().any(|option_name| {
            if let Some(rest) = option_name.strip_prefix("--") {
                rest == name
            } else if let Some(rest) = option_name.strip_prefix('-') {
                rest == name
            } else {
                false
            }
        })
    }
}

/// An `Argument` indicates when a Signature or an Opt takes a value as a parameter.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Argument {
    pub display_name: Option<String>,
    pub description: Option<String>,
    // If the argument can be repeated a variable number of times (such as git add [file name...])
    pub is_variadic: bool,
    // `argument_types` is a vector because an argument can derive suggestions from multiple places.
    // If empty, the  parser will accept any string.
    pub argument_types: Vec<ArgumentType>,
    pub optional: IsArgumentOptional,
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

    pub fn alias_by_name<'a>(
        &self,
        aliases: Option<&'a Aliases>,
        alias_name: &AliasName,
    ) -> Option<&'a Alias> {
        let aliases = match aliases {
            None => {
                log::error!(
                    "Argument {:?} specified alias {:?} but none are specified",
                    &self.display_name,
                    alias_name
                );
                return None;
            }
            Some(aliases) => aliases,
        };

        match aliases.get(alias_name) {
            None => {
                log::error!(
                    "Argument {:?} specified alias {:?} but it wasn't specified",
                    &self.display_name,
                    alias_name
                );
                None
            }
            Some(alias) => Some(alias),
        }
    }
}

type DefaultValue = String;
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum IsArgumentOptional {
    Optional(Option<DefaultValue>),
    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ArgumentType {
    Suggestion(Suggestion),
    Template(Template),
    Generator(GeneratorName),
    Alias(AliasName),
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
    CommandFromTokens(fn(&[&str]) -> String),
    ShellCommand(String),
}

impl Debug for GeneratorProcess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandFromTokens(_) => write!(f, "Context Generator"),
            Self::ShellCommand(s) => write!(f, "{}", s),
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
        shell_command: impl Into<String>,
        on_complete_callback: fn(&str) -> GeneratorResults,
    ) -> Self {
        Generator {
            process: GeneratorProcess::ShellCommand(shell_command.into()),
            on_complete_callback,
        }
    }

    pub fn command_from_tokens(
        command_from_tokens: fn(&[&str]) -> String,
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
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TemplateType {
    Files,
    Folders,
    #[allow(dead_code)]
    FilesAndFolders,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
pub struct AliasName(pub String);

/// An `Alias` takes in a shell command and expands it if there is any command specific aliases.
#[derive(Clone)]
pub struct Alias {
    // Given a list of command tokens, return the shell command that will generate aliases.
    pub command_with_tokens: fn(&[&str]) -> String,
    pub on_complete_callback: fn(&str) -> String,
}

impl Alias {
    pub fn command(&self, input: &[&str]) -> String {
        (self.command_with_tokens)(input)
    }

    pub fn on_complete(&self, input: &str) -> String {
        (self.on_complete_callback)(input)
    }
}
