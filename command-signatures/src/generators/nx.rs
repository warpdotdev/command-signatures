use serde_json::Result;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NxOutput {
    projects: HashMap<String, NxProject>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NxProject {
    project_type: String,
}

fn process_workspace_json(
    output: &str,
    filter_fn: fn(project: &(String, NxProject)) -> bool,
) -> GeneratorResults {
    let json_output: Result<NxOutput> = serde_json::from_str(output);
    match json_output {
        Ok(output) => output
            .projects
            .into_iter()
            .filter(filter_fn)
            .map(|(name, _)| Suggestion::new(name))
            .collect_unordered_results(),
        Err(e) => {
            log::info!("Unable to deserialize nx output: {:?}", e);
            GeneratorResults::default()
        }
    }
}

fn process_generators(output: &str) -> GeneratorResults {
    output
        .split('\n')
        .filter_map(|line| line.split(' ').next().map(Suggestion::new))
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("nx")
        .add_generator(
            "apps",
            Generator::script("cat workspace.json", |output| {
                process_workspace_json(output, |(name, project)| {
                    project.project_type == "application" && !name.ends_with("-e2e")
                })
            }),
        )
        .add_generator(
            "e2e_apps",
            Generator::script("cat workspace.json", |output| {
                process_workspace_json(output, |(name, project)| {
                    project.project_type == "application" && name.ends_with("-e2e")
                })
            }),
        )
        .add_generator(
            "apps_and_libs",
            Generator::script("cat workspace.json", |output| {
                process_workspace_json(output, |_| true)
            }),
        )
        .add_generator(
            "local_schematics",
            Generator::script("ls tools/schematics", process_generators),
        )
        .add_generator(
            "local_generators",
            Generator::script("ls tools/generators", process_generators),
        )
        .add_generator(
            "installed_plugins",
            Generator::script("nx list", |output| {
                if output.contains("Installed plugins") {
                    if let Some(installed_plugins) = output.split('>').nth(1) {
                        installed_plugins
                            .split('\n')
                            .skip(1)
                            .filter_map(|line| {
                                if !line.is_empty() {
                                    line.trim().split(' ').next().map(Suggestion::new)
                                } else {
                                    None
                                }
                            })
                            .collect_unordered_results()
                    } else {
                        GeneratorResults::default()
                    }
                } else {
                    GeneratorResults::default()
                }
            }),
        )
}
