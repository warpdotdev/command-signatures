use serde::Deserialize;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

const GET_COMMAND_NAMES: &str = "Get-Command -Type Cmdlet, Function, Alias | \
    Select-Object -Property Name, CommandType | \
    ForEach-Object { @{Name = $_.Name; Description = ($_.CommandType | Out-String -NoNewline)} } | \
    ConvertTo-Json";

const GET_PROCESS_NAMES: &str = "Get-Process | Select-Object -ExpandProperty Name | \
    Sort-Object | Get-Unique";

/// Exclude Warp internal variables here to avoid encouraging users to alter those. Doing so may
/// break their session.
const GET_VARIABLE_NAMES: &str = "Get-Variable | \
    Where-Object { -not $_.Name.ToLower().StartsWith('_warp') } | \
    Select-Object -ExpandProperty Name";

#[derive(Deserialize)]
struct SuggestionWithDescription {
    #[serde(alias = "Name")]
    name: String,

    #[serde(alias = "Description")]
    description: String,
}

impl From<SuggestionWithDescription> for Suggestion {
    fn from(value: SuggestionWithDescription) -> Self {
        Suggestion::with_description(value.name, value.description)
    }
}

fn process_suggestions_with_desc(output: &str) -> GeneratorResults {
    let commands = serde_json::from_str::<Vec<SuggestionWithDescription>>(output);
    commands
        .unwrap_or_default()
        .into_iter()
        .map(Into::into)
        .collect_unordered_results()
}

pub fn get_help_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Get-Help").add_generator(
        "get_command_names",
        Generator::script(GET_COMMAND_NAMES, process_suggestions_with_desc),
    )
}

fn process_plaintext_lines(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter(|val| !val.is_empty())
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn get_process_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Get-Process").add_generator(
        "get_process_names",
        Generator::script(GET_PROCESS_NAMES, process_plaintext_lines),
    )
}

pub fn debug_process_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Debug-Process").add_generator(
        "get_process_names",
        Generator::script(GET_PROCESS_NAMES, process_plaintext_lines),
    )
}

pub fn wait_process_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Wait-Process").add_generator(
        "get_process_names",
        Generator::script(GET_PROCESS_NAMES, process_plaintext_lines),
    )
}

pub fn enter_ps_host_process_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Enter-PSHostProcess").add_generator(
        "get_process_names",
        Generator::script(GET_PROCESS_NAMES, process_plaintext_lines),
    )
}

pub fn get_variable_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Get-Variable").add_generator(
        "get_variable_names",
        Generator::script(GET_VARIABLE_NAMES, process_plaintext_lines),
    )
}

pub fn clear_variable_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Clear-Variable").add_generator(
        "get_variable_names",
        Generator::script(GET_VARIABLE_NAMES, process_plaintext_lines),
    )
}

pub fn remove_variable_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Remove-Variable").add_generator(
        "get_variable_names",
        Generator::script(GET_VARIABLE_NAMES, process_plaintext_lines),
    )
}

pub fn set_variable_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("Set-Variable").add_generator(
        "get_variable_names",
        Generator::script(GET_VARIABLE_NAMES, process_plaintext_lines),
    )
}
