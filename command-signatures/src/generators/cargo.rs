use super::{fig_token, output_parsers, template_filters};
use std::collections::{HashMap, HashSet};

use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
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
            Generator::script(
                CommandBuilder::single_command("cargo metadata --no-deps --format-version 1"),
                |output| {
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
                },
            ),
        )
        .add_generator(
            "target_list",
            Generator::script(
                CommandBuilder::single_command("rustc --print target-list"),
                |output| {
                    output
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line, "target"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "dependencies",
            Generator::script(
                CommandBuilder::single_command("cargo metadata --format-version 1"),
                |output| {
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
                },
            ),
        )
        .add_generator(
            "bin_list",
            Generator::script(
                CommandBuilder::single_command("cargo metadata --no-deps --format-version 1"),
                |output| metadata_targets_of_kind(output, "bin"),
            ),
        )
        .add_generator(
            "test_targets",
            Generator::script(
                CommandBuilder::single_command("cargo metadata --no-deps --format-version 1"),
                |output| metadata_targets_of_kind(output, "test"),
            ),
        )
        .add_generator(
            "spec",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command(r#"cargo install --list"#),
                    CommandBuilder::single_command(
                        r#"\grep -E "^[a-zA-Z\\-]+\\sv" | cut -d ' ' -f 1"#,
                    ),
                ),
                |output| {
                    output
                        .lines()
                        .map(Suggestion::new)
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "read_manifest",
            Generator::script(
                CommandBuilder::single_command("cargo read-manifest"),
                output_parsers::cargo_read_manifest_bins,
            ),
        )
        .add_generator(
            "crates_io_search",
            Generator::command_from_tokens(
                fig_token::crates_io_search,
                output_parsers::json_crates,
            ),
        )
        .add_generator(
            "test_list",
            Generator::command_from_tokens(fig_token::cargo_test_list, output_parsers::named_lines),
        )
        .add_filter("filter-cargo-toml", template_filters::cargo_toml())
        .add_filter("filter-cargo-lock", template_filters::cargo_lock())
        .add_filter("filter-rustfmt-toml", template_filters::rustfmt_toml())
        .add_filter("filter-rs", template_filters::rs())
}

fn metadata_targets_of_kind(output: &str, kind: &str) -> GeneratorResults {
    match serde_json::from_str::<Metadata>(output) {
        Ok(metadata) => metadata
            .packages
            .into_iter()
            .flat_map(|package| package.targets.into_iter().flatten())
            .filter(|target| target.kind.iter().any(|item| item == kind))
            .map(|target| Suggestion::new(target.name))
            .collect_unordered_results(),
        Err(e) => {
            log::error!("Couldn't parse cargo metadata with error {e}");
            GeneratorResults::default()
        }
    }
}
