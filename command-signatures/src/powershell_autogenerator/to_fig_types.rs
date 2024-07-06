use itertools::Itertools as _;

use crate::fig_types::{Arg, Command, CommandOption, NameOrSuggestion};
use crate::powershell_autogenerator::CmdletHelp;

impl From<CmdletHelp> for Command {
    fn from(cmdlet_help: CmdletHelp) -> Self {
        let options = cmdlet_help
            .parameters
            .unwrap_or_default()
            .parameter
            .iter()
            .map(|param| {
                let mut name = vec![format!("-{}", param.name)];
                name.extend(param.aliases.as_ref().map(|alias| "-".to_owned() + alias));

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

                // "Switches", i.e. a flag without an argument, are either "SwitchParameter" or
                // "System.Management.Automation.SwitchParameter"
                let args = if param.type_info.name.ends_with("SwitchParameter") {
                    vec![]
                } else {
                    vec![Arg {
                        suggestions: suggestions
                            .and_then(|param| param.allowed_values.as_ref())
                            .map(|values| values.values.clone())
                            .unwrap_or_default()
                            .into_iter()
                            .map(NameOrSuggestion::Name)
                            .collect_vec(),
                        is_variadic: param.variable_length,
                        ..Default::default()
                    }]
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
                    description: param
                        .description
                        .iter()
                        .find(|para| !para.text.contains("> [!NOTE] >"))
                        .map(|pg| pg.text.clone()),
                    is_dangerous: false,
                    priority: None,
                    hidden: false,
                }
            })
            .collect_vec();
        Self {
            name: vec![cmdlet_help.name],
            // PowerShell cmdlets don't have subcommands.
            subcommands: vec![],
            options,
            // PowerShell cmdlets never require positional arguments. There are some parameters
            // which may be specified positionally, but they always have named flags as an
            // alternative. The named flags are generally encouraged.
            args: vec![],
            alias_name: cmdlet_help.aliases.get(0).map(|s| s.as_str().into()),
            additional_suggestions: vec![],
            description: Some(cmdlet_help.synopsis),
            is_dangerous: false,
            priority: None,
            hidden: false,
        }
    }
}
