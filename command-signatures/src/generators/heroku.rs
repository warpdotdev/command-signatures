use serde_json::Result;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

#[derive(serde::Deserialize)]
struct HerokuAppOutput {
    #[serde(default)]
    name: String,
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("heroku").add_generator(
        "get_app_generator",
        Generator::script(
            CommandBuilder::single_command("heroku apps --all --json"),
            |output| {
                let json_output: Result<Vec<HerokuAppOutput>> = serde_json::from_str(output);

                if let Ok(json_output) = json_output {
                    json_output
                        .into_iter()
                        .map(|heroku_output| Suggestion::new(heroku_output.name))
                        .collect_unordered_results()
                } else {
                    log::info!(
                        "Unable to deserialize heroku output {:?}",
                        json_output.err().unwrap()
                    );
                    GeneratorResults::default()
                }
            },
        ),
    )
}
