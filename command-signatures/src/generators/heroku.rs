use serde_json::Result;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

#[derive(serde::Deserialize)]
struct HerokuAppOutput {
    #[serde(default)]
    name: String,
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("heroku").add_generator(
        "get_app_generator",
        Generator::new("heroku apps --all --json", |output| {
            let json_output: Result<Vec<HerokuAppOutput>> = serde_json::from_str(output);

            if let Ok(json_output) = json_output {
                json_output
                    .into_iter()
                    .map(|heroku_output| Suggestion::new(heroku_output.name))
                    .collect_from_unordered_suggestions()
            } else {
                log::info!(
                    "Unable to deserialize heroku output {:?}",
                    json_output.err().unwrap()
                );
                GeneratorResults::empty()
            }
        }),
    )
}
