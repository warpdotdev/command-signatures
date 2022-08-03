use std::collections::HashMap;

use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

use serde_json::Result;

#[derive(serde::Deserialize)]
struct MetaData {
    #[serde(default)]
    packages: Vec<Package>,
}

#[derive(serde::Deserialize)]
struct Package {
    #[serde(default)]
    features: HashMap<String, Vec<String>>,
    name: String,
    description: Option<String>,
    targets: Option<Vec<Target>>,
}

#[derive(serde::Deserialize)]
struct Target {
    #[serde(default)]
    kind: Vec<String>,
    name: String,
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("cargo")
        .add_generator(
            "features_generators",
            Generator::new("cargo metadata --no-deps --format-version 1", |output| {
                let metadata: Result<MetaData> = serde_json::from_str(output);

                if let Ok(metadata) = metadata {
                    let mut suggestions = Vec::new();
                    for package in metadata.packages {
                        suggestions.extend(
                            package
                                .features
                                .keys()
                                .map(|feature| Suggestion::with_description(feature, "Feature")),
                        );
                    }
                    return suggestions.into_iter().collect_unordered_results();
                }
                GeneratorResults::default()
            }),
        )
        .add_generator(
            "target_list",
            Generator::new("rustc --print target-list", |output| {
                output
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(|line| Suggestion::with_description(line, "target"))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "dependencies",
            Generator::new("cargo metadata --format-version 1", |output| {
                let metadata: Result<MetaData> = serde_json::from_str(output);

                if let Ok(metadata) = metadata {
                    let mut items = metadata
                        .packages
                        .iter()
                        .map(|package| (package.name.clone(), package.description.clone()))
                        .collect::<Vec<(String, Option<String>)>>();

                    items.dedup();

                    return items
                        .into_iter()
                        .map(|(name, description)| match description {
                            Some(description) => Suggestion::with_description(name, description),
                            None => Suggestion::new(name),
                        })
                        .collect_unordered_results();
                }
                GeneratorResults::default()
            }),
        )
        .add_generator(
            "bin_list",
            Generator::new("cargo metadata --no-deps --format-version 1", |output| {
                let metadata: Result<MetaData> = serde_json::from_str(output);

                if let Ok(metadata) = metadata {
                    let mut suggestions = Vec::new();
                    let binary_key = "bin".to_string();
                    for package in metadata.packages {
                        if let Some(targets) = package.targets {
                            suggestions.extend(
                                targets
                                    .iter()
                                    .filter(|target| target.kind.contains(&binary_key))
                                    .map(|target| Suggestion::new(target.name.clone())),
                            );
                        }
                    }
                    return suggestions.into_iter().collect_unordered_results();
                }
                GeneratorResults::default()
            }),
        )
        .add_generator(
            "spec",
            Generator::new(
                "cargo install --list | \\grep -E \"^[a-zA-Z\\-]+\\sv\" | cut -d ' ' -f 1",
                |output| {
                    output
                        .lines()
                        .map(Suggestion::new)
                        .collect_unordered_results()
                },
            ),
        )
}
