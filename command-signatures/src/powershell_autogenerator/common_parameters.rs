use warp_completion_metadata::fig_types::{Arg, CommandOption, NameOrSuggestion};

/// These were transcribed from this page for PowerShell version 7.4
/// https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_commonparameters
pub(super) fn cmdlet_common_parameters() -> [CommandOption; 12] {
    [
        CommandOption {
            name: vec!["-Debug".to_string(), "-db".to_string()],
            description: Some(
                "Displays programmer-level detail about the operation done by the command. This \
                parameter works only when the command generates a debugging message."
                    .to_string(),
            ),
            ..Default::default()
        },
        CommandOption {
            name: vec!["-ErrorAction".to_string(), "-ea".to_string()],
            description: Some(
                "Determines how the cmdlet responds to a non-terminating error from the command. \
                This parameter works only when the command generates a non-terminating error."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.Management.Automation.ActionPreference".to_string()),
                suggestions: vec![
                    NameOrSuggestion::Name("Break".to_string()),
                    NameOrSuggestion::Name("Suspend".to_string()),
                    NameOrSuggestion::Name("Ignore".to_string()),
                    NameOrSuggestion::Name("Inquire".to_string()),
                    NameOrSuggestion::Name("Continue".to_string()),
                    NameOrSuggestion::Name("Stop".to_string()),
                    NameOrSuggestion::Name("SilentlyContinue".to_string()),
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-ErrorVariable".to_string(), "-ev".to_string()],
            description: Some(
                "Error records are automatically stored in the `$Error` automatic variable. When \
                you use the ErrorVariable parameter on a command, PowerShell also stores the error \
                records emitted by the command in the variable specified by the parameter."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.String".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-InformationAction".to_string(), "-infa".to_string()],
            description: Some(
                "Within the command or script in which it's used, the InformationAction common \
                parameter overrides the value of the `$InformationPreference` preference variable, \
                which by default is set to SilentlyContinue."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.Management.Automation.ActionPreference".to_string()),
                suggestions: vec![
                    NameOrSuggestion::Name("Break".to_string()),
                    NameOrSuggestion::Name("Suspend".to_string()),
                    NameOrSuggestion::Name("Ignore".to_string()),
                    NameOrSuggestion::Name("Inquire".to_string()),
                    NameOrSuggestion::Name("Continue".to_string()),
                    NameOrSuggestion::Name("Stop".to_string()),
                    NameOrSuggestion::Name("SilentlyContinue".to_string()),
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-InformationVariable".to_string(), "-iv".to_string()],
            description: Some(
                "When you use the InformationVariable common parameter, information records are \
                stored in the variable specified by the parameter."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.String".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-OutBuffer".to_string(), "-ob".to_string()],
            description: Some(
                "Determines the number of objects to accumulate in a buffer before any objects are \
                sent through the pipeline. If you omit this parameter, objects are sent as they're \
                generated."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.Int32".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-OutVariable".to_string(), "-ov".to_string()],
            description: Some(
                "Stores output objects from the command in the specified variable in addition to \
                sending the output along the pipeline."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.String".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-PipelineVariable".to_string(), "-pv".to_string()],
            description: Some(
                "PipelineVariable allows access to the most recent value passed into the next \
                pipeline segment by the command that uses this parameter. Any command in the \
                pipeline can access the value using the named PipelineVariable. The value is \
                assigned to the variable when it's passed into the next pipeline segment."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.String".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-ProgressAction".to_string(), "-proga".to_string()],
            description: Some(
                "Determines how PowerShell responds to progress updates generated by a script, \
                cmdlet, or provider, such as the progress bars generated by the Write-Progress \
                cmdlet. The Write-Progress cmdlet creates progress bars that show a command's \
                status."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.Management.Automation.ActionPreference".to_string()),
                suggestions: vec![
                    NameOrSuggestion::Name("Break".to_string()),
                    NameOrSuggestion::Name("Suspend".to_string()),
                    NameOrSuggestion::Name("Ignore".to_string()),
                    NameOrSuggestion::Name("Inquire".to_string()),
                    NameOrSuggestion::Name("Continue".to_string()),
                    NameOrSuggestion::Name("Stop".to_string()),
                    NameOrSuggestion::Name("SilentlyContinue".to_string()),
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-Verbose".to_string(), "-vb".to_string()],
            description: Some(
                "Displays detailed information about the operation done by the command. This \
                information resembles the information in a trace or in a transaction log. This \
                parameter works only when the command generates a verbose message."
                    .to_string(),
            ),
            ..Default::default()
        },
        CommandOption {
            name: vec!["-WarningAction".to_string(), "-wa".to_string()],
            description: Some(
                "Determines how the cmdlet responds to a warning from the command. Continue is the \
                default value. This parameter works only when the command generates a warning \
                message."
                    .to_string(),
            ),
            args: vec![Arg {
                name: Some("System.Management.Automation.ActionPreference".to_string()),
                suggestions: vec![
                    NameOrSuggestion::Name("Break".to_string()),
                    NameOrSuggestion::Name("Suspend".to_string()),
                    NameOrSuggestion::Name("Ignore".to_string()),
                    NameOrSuggestion::Name("Inquire".to_string()),
                    NameOrSuggestion::Name("Continue".to_string()),
                    NameOrSuggestion::Name("Stop".to_string()),
                    NameOrSuggestion::Name("SilentlyContinue".to_string()),
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        CommandOption {
            name: vec!["-WarningVariable".to_string(), "-wv".to_string()],
            description: Some(
                "Stores warning records about the command in the specified variable.".to_string(),
            ),
            args: vec![Arg {
                name: Some("System.String".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },
    ]
}
