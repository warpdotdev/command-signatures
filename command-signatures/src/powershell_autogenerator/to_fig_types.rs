use std::{collections::HashMap, ops::Not};

use itertools::Itertools;
use warp_completion_metadata::fig_types::{ParserDirectives, StringOrNumber, Suggestion};

use crate::powershell_autogenerator::{common_parameters::cmdlet_common_parameters, CmdletHelp};
use crate::{
    fig_types::{Arg, Command, CommandOption, NameOrSuggestion},
    powershell_autogenerator::ParameterPosition,
};

impl From<CmdletHelp> for Command {
    fn from(cmdlet_help: CmdletHelp) -> Self {
        let parameters = &cmdlet_help.parameters.unwrap_or_default().parameter;
        // In PowerShell, all options are first-and-foremost named options. _Some_ options may
        // instead be provided as positional args. As we loop through the named options, we'll
        // collect those in this HashMap. Why a HashMap? Sometimes, multiple different params may
        // appear in a particular arg position depending on the "syntax". For example, see
        // `Get-Help Add-Member` in the "SYNTAX" section. This HashMap is keyed by the position
        // where the value is all params that _may_ appear in that position.
        let mut top_level_args = HashMap::<usize, Vec<Arg>>::new();

        let mut options = parameters
            .iter()
            .map(|param| {
                let mut name = vec![format!("-{}", param.name)];
                name.extend(param.aliases.iter().map(|alias| format!("-{}", alias)));

                // For some reason, [`crate::powershell_autogenerator::Parameter::allowed_values`]
                // is always None inside [`CmdletHelp::parameters`], but it is defined inside
                // [`CmdletHelp::syntax`], so we look for the matching parameter there.
                let suggestions = cmdlet_help
                    .syntax
                    .syntax_items
                    .iter()
                    .flat_map(|item| &item.parameter)
                    .find(|syn_param| {
                        syn_param.name == param.name
                            && syn_param
                                .allowed_values
                                .as_ref()
                                .is_some_and(|values| !values.values.is_empty())
                    });

                let description = param
                    .description
                    .iter()
                    .find(|param| !param.text.contains("> [!NOTE] >"))
                    .map(|pg| pg.text.clone());

                let type_name = &param.type_info.name;

                // "Switches", i.e. a flag without an argument, are either "SwitchParameter",
                // "System.Management.Automation.SwitchParameter", or "switch".
                let args = if type_name.ends_with("SwitchParameter")
                    || type_name.to_lowercase() == "switch"
                {
                    vec![]
                } else {
                    let arg = Arg {
                        name: Some(type_name.clone()),
                        default: param.default_value.clone().map(StringOrNumber::String),
                        // TODO(CORE-2677) Recognize PowerShell array syntax.
                        is_variadic: false,
                        suggestions: suggestions
                            .and_then(|param| param.allowed_values.as_ref())
                            .map(|values| values.values.clone())
                            .unwrap_or_default()
                            .into_iter()
                            .map(|name| {
                                NameOrSuggestion::Suggestion(Suggestion {
                                    name: vec![name],
                                    ..Default::default()
                                })
                            })
                            .collect_vec(),
                        ..Default::default()
                    };
                    if let ParameterPosition::Index(i) = &param.position {
                        let arg = Arg {
                            name: name.first().cloned(),
                            description: description.clone(),
                            // TODO(CORE-2677) Recognize PowerShell array syntax.
                            is_variadic: false,
                            suggestions: arg.suggestions.clone(),
                            ..Default::default()
                        };
                        top_level_args.entry(*i).or_default().push(arg);
                    }
                    vec![arg]
                };

                CommandOption {
                    name,
                    args,
                    // This only applies to subcommands, which PowerShell cmdlets don't have.
                    is_persistent: false,
                    is_required: param.required,
                    // PowerShell cmdlets forbid this.
                    requires_equals: false,
                    // PowerShell cmdlets forbid this too.
                    is_repeatable: None,
                    // Difficult to parse.
                    exclusive_on: vec![],
                    // Difficult to parse.
                    depends_on: vec![],
                    description,
                    is_dangerous: false,
                    priority: None,
                    hidden: false,
                }
            })
            .collect_vec();

        options.extend(cmdlet_common_parameters());

        let args = top_level_args
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .map(|(_, params)| match params.into_iter().exactly_one() {
                // If there is unambiguously one parameter in this position, use that parameter
                // definition as-is.
                Ok(param) => param,
                // If it's ambiguous, create a new parameter definition which awkwardly combines
                // the names of the multiple possible parameters and has no other information.
                Err(e) => {
                    let combined_names = e
                        .into_iter()
                        .map(|arg| arg.name.unwrap_or("-Unnamed".to_string()))
                        .join("-Or");
                    Arg {
                        name: combined_names.is_empty().not().then_some(combined_names),
                        ..Default::default()
                    }
                }
            })
            .collect_vec();

        Self {
            name: vec![cmdlet_help.name],
            // PowerShell cmdlets don't have subcommands.
            subcommands: vec![],
            options,
            args,
            alias_generator: None,
            additional_suggestions: vec![],
            description: Some(cmdlet_help.synopsis),
            is_dangerous: false,
            priority: None,
            hidden: false,
            parser_directives: ParserDirectives {
                // All cmdlet flags are prefixed by a single hyphen.
                flags_are_posix_noncompliant: true,
                flags_match_unique_prefix: true,
                always_case_insensitive: true,
            },
        }
    }
}
