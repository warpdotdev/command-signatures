use std::collections::{HashMap, HashSet};

use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

use serde_json::Result;

/// The output of cargo metadata. It should contain
/// a list of resolved dependencies of a package.
#[derive(serde::Deserialize)]
struct Metadata {
    #[serde(default)]
    packages: Vec<Package>,
}

/// The fields of a cargo package JSON.
#[derive(serde::Deserialize)]
struct Package {
    #[serde(default)]
    /// List of feature flags.
    features: HashMap<String, Vec<String>>,
    /// Name of the package.
    name: String,
    /// Description of the package.
    description: Option<String>,
    /// List of compilation targets.
    targets: Option<Vec<Target>>,
}

#[derive(serde::Deserialize)]
struct Target {
    #[serde(default)]
    kind: Vec<String>,
    name: String,
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("cargo")
        .add_generator(
            "features_generators",
            Generator::script("cargo metadata --no-deps --format-version 1", |output| {
                let metadata: Result<Metadata> = serde_json::from_str(output);

                match metadata {
                    Ok(metadata) => metadata
                        .packages
                        .into_iter()
                        .flat_map(|package| package.features.into_keys())
                        .map(|feature| Suggestion::with_description(feature, "Feature"))
                        .collect_unordered_results(),
                    Err(e) => {
                        log::error!("Couldn't parse cargo metadata with error {}", e);
                        GeneratorResults::default()
                    }
                }
            }),
        )
        .add_generator(
            "target_list",
            Generator::script("rustc --print target-list", |output| {
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
            Generator::script("cargo metadata --format-version 1", |output| {
                let metadata: Result<Metadata> = serde_json::from_str(output);

                match metadata {
                    Ok(metadata) => {
                        let items = metadata
                            .packages
                            .iter()
                            .map(|package| (package.name.clone(), package.description.clone()))
                            .collect::<HashSet<(String, Option<String>)>>();

                        items
                            .into_iter()
                            .map(|(name, description)| match description {
                                Some(description) => {
                                    Suggestion::with_description(name, description)
                                }
                                None => Suggestion::new(name),
                            })
                            .collect_unordered_results()
                    }
                    Err(e) => {
                        log::error!("Couldn't parse cargo metadata with error {}", e);
                        GeneratorResults::default()
                    }
                }
            }),
        )
        .add_generator(
            "bin_list",
            Generator::script("cargo metadata --no-deps --format-version 1", |output| {
                let metadata: Result<Metadata> = serde_json::from_str(output);

                match metadata {
                    Ok(metadata) => metadata
                        .packages
                        .into_iter()
                        .flat_map(|package| package.targets.into_iter().flatten())
                        .filter_map(|target| {
                            target
                                .kind
                                .into_iter()
                                .any(|item| item == "bin")
                                .then(|| Suggestion::new(target.name))
                        })
                        .collect_unordered_results(),
                    Err(e) => {
                        log::error!("Couldn't parse cargo metadata with error {}", e);
                        GeneratorResults::default()
                    }
                }
            }),
        )
        .add_generator(
            "spec",
            Generator::script(
                r#"cargo install --list | \grep -E "^[a-zA-Z\\-]+\\sv" | cut -d ' ' -f 1"#,
                |output| {
                    output
                        .lines()
                        .map(Suggestion::new)
                        .collect_unordered_results()
                },
            ),
        )
}
