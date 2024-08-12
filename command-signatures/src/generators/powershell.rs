use serde::Deserialize;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

const GET_COMMAND_NAMES: &str = "Get-Command -Type Cmdlet, Function, Alias | \
    Select-Object -Property Name, CommandType | \
    ForEach-Object { @{Name = $_.Name; Description = ($_.CommandType | Out-String -NoNewline)} } | \
    ConvertTo-Json";

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

fn get_command_names(output: &str) -> GeneratorResults {
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
        Generator::script(GET_COMMAND_NAMES, get_command_names),
    )
}
