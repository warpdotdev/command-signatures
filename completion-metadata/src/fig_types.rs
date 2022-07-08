use crate::{
    Argument, ArgumentType, GeneratorName, Importance, IsArgumentOptional, Opt, Order, Priority,
    Signature,
};
use serde::{Deserialize, Serialize};
use serde_with::formats::PreferMany;
use serde_with::{serde_as, NoneAsEmptyString, OneOrMany};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

/// https://fig.io/docs/reference/suggestion/indicating-priority
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct FigPriority(pub u32);

/// Mapping to the exact types of Fig's completion specs at commit
/// 3eb3450c5b54de3a2aa31737035e616361d59573.
/// See https://github.com/withfig/autocomplete-tools/blob/3eb3450c5b54de3a2aa31737035e616361d59573/packages/autocomplete-types/index.d.ts
/// for the original type definition.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SuggestionType {
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "arg")]
    Arg,
    #[serde(rename = "subcommand")]
    Subcommand,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "shortcut")]
    Shortcut,
}

#[serde_as]
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Suggestion {
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub name: Vec<String>,

    #[serde(default)]
    #[serde(rename = "type")]
    pub suggestion_type: Option<SuggestionType>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    is_dangerous: bool,

    #[serde(default)]
    priority: Option<FigPriority>,

    #[serde(default)]
    hidden: bool,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Command {
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub name: Vec<String>,

    #[serde(default)]
    pub subcommands: Vec<Command>,

    #[serde(default)]
    pub options: Vec<CommandOption>,

    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<Arg>,

    #[serde(default)]
    pub additional_suggestions: Vec<Suggestion>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    is_dangerous: bool,

    #[serde(default)]
    priority: Option<FigPriority>,

    #[serde(default)]
    hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumberOrBool {
    Number(usize),
    Bool(bool),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommandOption {
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub name: Vec<String>,

    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<Arg>,

    #[serde(default)]
    #[serde(rename = "isPersistent")]
    is_persistent: bool,

    #[serde(default)]
    #[serde(rename = "isRequired")]
    pub is_required: bool,

    #[serde(default)]
    #[serde(rename = "requiresEquals")]
    pub requires_equals: bool,

    // TODO: we should be using this option to determine if an option can be repeated.
    #[serde(default)]
    #[serde(rename = "isRepeatable")]
    pub is_repeatable: Option<NumberOrBool>,

    #[serde(default)]
    #[serde(rename = "exclusiveOn")]
    pub exclusive_on: Vec<String>,

    #[serde(default)]
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    is_dangerous: bool,

    #[serde(default)]
    priority: Option<FigPriority>,

    #[serde(default)]
    hidden: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Arg {
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    pub description: Option<String>,

    #[serde(default)]
    #[serde(rename = "isDangerous")]
    pub is_dangerous: bool,

    #[serde(default)]
    pub suggestions: Vec<NameOrSuggestion>,

    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "generatorName")]
    pub generator_name: Vec<GeneratorName>,

    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub template: Vec<Template>,

    #[serde(default)]
    #[serde(rename = "isVariadic")]
    pub is_variadic: bool,

    #[serde(default)]
    #[serde(rename = "isOptional")]
    pub is_optional: bool,

    #[serde(default)]
    #[serde(rename = "isCommand")]
    pub is_command: bool,

    /// The default value for an optional argument. This is just a string.
    #[serde(default)]
    pub default: Option<StringOrNumber>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(usize),
}

impl From<StringOrNumber> for String {
    fn from(string_or_number: StringOrNumber) -> Self {
        match string_or_number {
            StringOrNumber::String(s) => s,
            StringOrNumber::Number(number) => number.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Template {
    #[serde(rename = "filepaths")]
    FilePaths,

    #[serde(rename = "folders")]
    Folders,

    #[serde(rename = "history")]
    History,
}

impl From<String> for Suggestion {
    fn from(name: String) -> Self {
        Suggestion {
            name: vec![name],
            ..Default::default()
        }
    }
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name,)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NameOrSuggestion {
    Name(String),
    Suggestion(Suggestion),
}

impl From<NameOrSuggestion> for Vec<crate::Suggestion> {
    fn from(name_or_suggestion: NameOrSuggestion) -> Self {
        match name_or_suggestion {
            NameOrSuggestion::Name(name) => vec![crate::Suggestion::new(name)],
            NameOrSuggestion::Suggestion(suggestion) => suggestion.into(),
        }
    }
}

impl From<Suggestion> for NameOrSuggestion {
    fn from(suggestion: Suggestion) -> Self {
        NameOrSuggestion::Suggestion(suggestion)
    }
}

impl From<Command> for Signature {
    fn from(command: Command) -> Self {
        Signature {
            name: command.name.first().cloned().unwrap_or_default(),
            description: command.description,
            arguments: if command.args.is_empty() {
                None
            } else {
                Some(command.args.into_iter().map(|a| a.into()).collect())
            },
            subcommands: if command.subcommands.is_empty() {
                None
            } else {
                Some(command.subcommands.into_iter().map(|s| s.into()).collect())
            },
            options: if command.options.is_empty() {
                None
            } else {
                Some(command.options.into_iter().map(|o| o.into()).collect())
            },
            priority: command.priority.map_or_else(Priority::default, Into::into),
        }
    }
}

impl From<Arg> for Argument {
    fn from(arg: Arg) -> Self {
        // The order of argument_types will dictate the order in which completions will be surfaced.
        // Currently, the order is Generators followed by Suggestions followed by Templates.
        // If there are multiple of any of the three types, the order in which they
        // are listed in the spec is the order in which they will be grouped.
        // For example, if argument_types = vec![generator_a, generator_b], then generator_a
        // completions will come before generator_b completions.
        let argument_types = arg
            .generator_name
            .into_iter()
            .map(ArgumentType::Generator)
            .chain(
                arg.suggestions
                    .into_iter()
                    .flat_map(Vec::from)
                    .map(ArgumentType::Suggestion),
            )
            .chain(arg.template.into_iter().filter_map(|template| {
                crate::Template::try_from(template)
                    .ok()
                    .map(ArgumentType::Template)
            }))
            .collect();

        let optional = if arg.is_optional {
            IsArgumentOptional::Optional(arg.default.map(Into::into))
        } else {
            IsArgumentOptional::Required
        };

        Argument {
            display_name: arg.name,
            description: arg.description,
            is_variadic: arg.is_variadic,
            argument_types,
            optional,
        }
    }
}

/// https://fig.io/docs/reference/suggestion/indicating-priority
/// 50 is default, so < 50 is Lower and > 50 is Higher
impl From<FigPriority> for Priority {
    fn from(priority: FigPriority) -> Self {
        let order = Order(priority.0).normalized();
        let default_order = Order(50);
        match order.cmp(&default_order) {
            Ordering::Less => Priority::Global(Importance::Less(order)),
            Ordering::Greater => Priority::Global(Importance::More(order)),
            Ordering::Equal => Priority::Default,
        }
    }
}

impl From<Suggestion> for Vec<crate::Suggestion> {
    fn from(suggestion: Suggestion) -> Self {
        suggestion
            .name
            .into_iter()
            .map(|name| crate::Suggestion {
                exact_string: name,
                description: suggestion.description.clone(),
                priority: suggestion.priority.map_or_else(Priority::default, Into::into),
            })
            .collect()
    }
}

impl From<CommandOption> for Opt {
    fn from(option: CommandOption) -> Self {
        Opt {
            exact_string: option.name,
            description: option.description,
            arguments: if option.args.is_empty() {
                None
            } else {
                Some(option.args.into_iter().map(|a| a.into()).collect())
            },
            required: option.is_required,
            priority: option.priority.map_or_else(Priority::default, Into::into),
        }
    }
}

impl TryFrom<Template> for crate::Template {
    type Error = ();

    fn try_from(template: Template) -> Result<Self, Self::Error> {
        match template {
            Template::FilePaths => Ok(crate::Template::Files),
            Template::Folders => Ok(crate::Template::Folders),
            Template::History => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fig_types::{
        Arg, Command, CommandOption, FigPriority, NameOrSuggestion, StringOrNumber, Suggestion,
    };

    use crate::{Importance, Order, Priority};

    #[test]
    fn deserialize_command() {
        let signature_json = r#"{
       "name":"defaults",
       "description":"Command line interface to a user's defaults.",
       "subcommands":[
          {
             "name":"read",
             "description":"shows defaults",
             "args":[
                {
                   "name":"domain",
                   "suggestions":[
                      {
                         "name":"-globalDomain",
                         "description":"Global domain"
                      },
                      {
                         "name":"-app",
                         "insertValue":"-app '{cursor}'",
                         "description":"Application name"
                      }
                   ]
                },
                {
                   "name":"key"
                }
             ]
          },
          {
             "name":"write",
             "insertValue":"write ",
             "description":"writes key for domain",
             "args":[
                {
                   "name":"domain",
                   "suggestions":[
                      {
                         "name":"-globalDomain",
                         "description":"Global domain"
                      },
                      {
                         "name":"-app",
                         "insertValue":"-app '{cursor}'",
                         "description":"Application name"
                      }
                   ]
                },
                {
                   "name":"key"
                },
                {
                   "name":"value"
                }
             ]
          },
          {
             "name":"delete",
             "description":"deletes domain or key in domain",
             "args":[
                {
                   "name":"domain",
                   "suggestions":[
                      {
                         "name":"-globalDomain",
                         "description":"Global domain"
                      },
                      {
                         "name":"-app",
                         "insertValue":"-app '{cursor}'",
                         "description":"Application name"
                      }
                   ]
                },
                {
                   "name":"key"
                }
             ]
          },
          {
             "name":"rename",
             "description":"renames old_key to new_key",
             "args":[
                {
                   "name":"domain",
                   "suggestions":[
                      {
                         "name":"-globalDomain",
                         "description":"Global domain"
                      },
                      {
                         "name":"-app",
                         "insertValue":"-app '{cursor}'",
                         "description":"Application name"
                      }
                   ]
                },
                {
                   "name":"old_key"
                },
                {
                   "name":"new_key"
                }
             ]
          },
          {
             "name":"domains",
             "description":"lists all domains"
          }
       ]
    }"#;

        let command: Command = serde_json::from_str(signature_json).unwrap();
        assert_eq!(
            command,
            Command {
                name: vec!["defaults".into()],
                description: Some("Command line interface to a user's defaults.".into()),
                is_dangerous: false,
                priority: None,
                hidden: false,
                subcommands: vec![
                    Command {
                        name: vec!["read".into()],
                        description: Some("shows defaults".into()),
                        is_dangerous: false,
                        priority: None,
                        hidden: false,
                        subcommands: vec![],
                        options: vec![],
                        args: vec![
                            Arg {
                                name: Some("domain".into()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![
                                    Suggestion {
                                        name: vec!["-globalDomain".into()],
                                        suggestion_type: None,
                                        description: Some("Global domain".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                    Suggestion {
                                        name: vec!["-app".into()],
                                        suggestion_type: None,
                                        description: Some("Application name".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                ],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("key".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            }
                        ],
                        additional_suggestions: vec![]
                    },
                    Command {
                        name: vec!["write".into()],
                        description: Some("writes key for domain".into()),
                        is_dangerous: false,
                        priority: None,
                        hidden: false,
                        subcommands: vec![],
                        options: vec![],
                        args: vec![
                            Arg {
                                name: Some("domain".into()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![
                                    Suggestion {
                                        name: vec!["-globalDomain".into()],
                                        suggestion_type: None,
                                        description: Some("Global domain".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                    Suggestion {
                                        name: vec!["-app".into()],
                                        suggestion_type: None,
                                        description: Some("Application name".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                ],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("key".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("value".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            }
                        ],
                        additional_suggestions: vec![]
                    },
                    Command {
                        name: vec!["delete".into()],
                        description: Some("deletes domain or key in domain".into()),
                        is_dangerous: false,
                        priority: None,
                        hidden: false,
                        subcommands: vec![],
                        options: vec![],
                        args: vec![
                            Arg {
                                name: Some("domain".into()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![
                                    Suggestion {
                                        name: vec!["-globalDomain".into()],
                                        suggestion_type: None,
                                        description: Some("Global domain".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                    Suggestion {
                                        name: vec!["-app".into()],
                                        suggestion_type: None,
                                        description: Some("Application name".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                ],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("key".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            }
                        ],
                        additional_suggestions: vec![]
                    },
                    Command {
                        name: vec!["rename".into()],
                        description: Some("renames old_key to new_key".into()),
                        is_dangerous: false,
                        priority: None,
                        hidden: false,
                        subcommands: vec![],
                        options: vec![],
                        args: vec![
                            Arg {
                                name: Some("domain".into()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![
                                    Suggestion {
                                        name: vec!["-globalDomain".into()],
                                        suggestion_type: None,
                                        description: Some("Global domain".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                    Suggestion {
                                        name: vec!["-app".into()],
                                        suggestion_type: None,
                                        description: Some("Application name".into()),
                                        is_dangerous: false,
                                        priority: None,
                                        hidden: false
                                    }
                                    .into(),
                                ],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("old_key".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            },
                            Arg {
                                name: Some("new_key".to_string()),
                                description: None,
                                is_dangerous: false,
                                suggestions: vec![],
                                generator_name: vec![],
                                template: vec![],
                                is_variadic: false,
                                is_optional: false,
                                is_command: false,
                                default: None
                            }
                        ],
                        additional_suggestions: vec![]
                    },
                    Command {
                        name: vec!["domains".into()],
                        description: Some("lists all domains".to_string()),
                        is_dangerous: false,
                        priority: None,
                        hidden: false,
                        subcommands: vec![],
                        options: vec![],
                        args: vec![],
                        additional_suggestions: vec![]
                    },
                ],
                options: vec![],
                args: vec![],
                additional_suggestions: vec![]
            }
        )
    }

    #[test]
    fn default_deserialiazes_as_string_or_number() {
        let json_string = r#"{
                            "name": "--cluster-storage-type",
                            "description": "Storage class for the cluster. _CLUSTER_STORAGE_TYPE_ must be one of: *hdd*, *ssd*.",
                            "args": {
                                "name": "CLUSTER_STORAGE_TYPE",
                                "description": "googlecloudsdk.calliope.base:_ChoiceValueType",
                                "suggestions": [
                                    "hdd",
                                    "ssd"
                                ],
                                "default": 8100
                            }
                        }"#;

        let cli_opt: CommandOption = serde_json::from_str(json_string).unwrap();
        assert_eq!(
            cli_opt.args.get(0).unwrap().default,
            Some(StringOrNumber::Number(8100))
        );

        let json_string = r#"{
                            "name": "--cluster-storage-type",
                            "description": "Storage class for the cluster. _CLUSTER_STORAGE_TYPE_ must be one of: *hdd*, *ssd*.",
                            "args": {
                                "name": "CLUSTER_STORAGE_TYPE",
                                "description": "googlecloudsdk.calliope.base:_ChoiceValueType",
                                "suggestions": [
                                    "hdd",
                                    "ssd"
                                ],
                                "default": "8100"
                            }
                        }"#;

        let cli_opt: CommandOption = serde_json::from_str(json_string).unwrap();
        assert_eq!(
            cli_opt.args.get(0).unwrap().default,
            Some(StringOrNumber::String("8100".into()))
        );
    }

    #[test]
    fn deserialize_option() {
        // Test suggestions represented as a string are deserialized correctly.
        let json_string = r#"{
                            "name": "--cluster-storage-type",
                            "description": "Storage class for the cluster. _CLUSTER_STORAGE_TYPE_ must be one of: *hdd*, *ssd*.",
                            "args": {
                                "name": "CLUSTER_STORAGE_TYPE",
                                "description": "googlecloudsdk.calliope.base:_ChoiceValueType",
                                "suggestions": [
                                    "hdd",
                                    "ssd"
                                ]
                            }
                        }"#;

        let cli_opt: CommandOption = serde_json::from_str(json_string).unwrap();
        assert_eq!(
            cli_opt.args.get(0).unwrap().suggestions,
            vec![
                NameOrSuggestion::Name("hdd".into()),
                NameOrSuggestion::Name("ssd".into())
            ]
        );

        // Test suggestions represented as an object are deserialized correctly.
        let json_string = r#"{
                            "name": "--cluster-storage-type",
                            "description": "Storage class for the cluster. _CLUSTER_STORAGE_TYPE_ must be one of: *hdd*, *ssd*.",
                            "args": {
                                "name": "CLUSTER_STORAGE_TYPE",
                                "description": "googlecloudsdk.calliope.base:_ChoiceValueType",
                                "suggestions": [
                                    { "name" : "hdd", "description": "hdd" },
                                    { "name" : "ssd", "description": "ssd" }
                                ]
                            }
                        }"#;

        let cli_opt: CommandOption = serde_json::from_str(json_string).unwrap();
        assert_eq!(
            cli_opt.args.get(0).unwrap().suggestions,
            vec![
                Suggestion {
                    name: vec!["hdd".into()],
                    suggestion_type: None,
                    description: Some("hdd".into()),
                    is_dangerous: false,
                    priority: None,
                    hidden: false
                }
                .into(),
                Suggestion {
                    name: vec!["ssd".into()],
                    suggestion_type: None,
                    description: Some("ssd".into()),
                    is_dangerous: false,
                    priority: None,
                    hidden: false
                }
                .into(),
            ]
        );

        // Test suggestions where are some are represented as a string and others are represented as
        // an object deserialize correctly.
        let json_string = r#"{
                            "name": "--cluster-storage-type",
                            "description": "Storage class for the cluster. _CLUSTER_STORAGE_TYPE_ must be one of: *hdd*, *ssd*.",
                            "args": {
                                "name": "CLUSTER_STORAGE_TYPE",
                                "description": "googlecloudsdk.calliope.base:_ChoiceValueType",
                                "suggestions": [
                                    { "name" : "hdd", "description": "hdd" },
                                    "ssd"
                                ]
                            }
                        }"#;

        let cli_opt: CommandOption = serde_json::from_str(json_string).unwrap();
        assert_eq!(
            cli_opt.args.get(0).unwrap().suggestions,
            vec![
                NameOrSuggestion::Suggestion(Suggestion {
                    name: vec!["hdd".into()],
                    suggestion_type: None,
                    description: Some("hdd".into()),
                    is_dangerous: false,
                    priority: None,
                    hidden: false
                }),
                NameOrSuggestion::Name("ssd".into()),
            ]
        );
    }

    #[test]
    fn test_from_fig_priority() {
        assert_eq!(Priority::from(FigPriority(50)), Priority::Default);

        assert_eq!(
            Priority::from(FigPriority(56)),
            Priority::Global(Importance::More(Order(56)))
        );
        assert_eq!(
            Priority::from(FigPriority(200)),
            Priority::Global(Importance::More(Order(100)))
        );

        assert_eq!(
            Priority::from(FigPriority(46)),
            Priority::Global(Importance::Less(Order(46)))
        );
        assert_eq!(
            Priority::from(FigPriority(0)),
            Priority::Global(Importance::Less(Order(1)))
        );
    }

    #[test]
    fn test_default_priority() {
        let fig_suggestion = Suggestion {
            name: vec!["first".into()],
            suggestion_type: None,
            description: Some("hdd".to_owned()),
            is_dangerous: false,
            priority: None,
            hidden: false,
        };

        let warp_suggestion = Vec::<crate::Suggestion>::from(fig_suggestion);

        assert_eq!(warp_suggestion.first().unwrap().priority, Priority::Default);
    }

    #[test]
    fn test_fig_suggestion_into_warp_suggestions() {
        let description = Some("hdd".into());
        let priority = Priority::Global(Importance::Less(Order(42)));
        let fig_suggestion = Suggestion {
            name: vec!["first".into(), "second".into()],
            suggestion_type: None,
            description: description.clone(),
            is_dangerous: false,
            priority: Some(FigPriority(42)),
            hidden: false,
        };

        let warp_suggestions = vec![
            crate::Suggestion {
                exact_string: "first".into(),
                description: description.clone(),
                priority,
            },
            crate::Suggestion {
                exact_string: "second".into(),
                description,
                priority,
            },
        ];

        assert_eq!(
            Vec::<crate::Suggestion>::from(fig_suggestion),
            warp_suggestions
        );
    }
}
