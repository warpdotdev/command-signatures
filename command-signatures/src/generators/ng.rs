use serde_json::Result;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AngularConfigOutput {
    project_type: String,
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ng").add_generator(
        "list_projects",
        Generator::script("ng config projects", |output| {
            let json_output: Result<HashMap<String, AngularConfigOutput>> =
                serde_json::from_str(output);
            match json_output {
                Ok(projects) => projects
                    .into_iter()
                    .map(|(project_name, config_output)| {
                        Suggestion::with_description(project_name, config_output.project_type)
                    })
                    .collect_unordered_results(),
                Err(e) => {
                    log::info!("Unable to deserialize angular output {:?}", e);
                    GeneratorResults::default()
                }
            }
        }),
    )
}
