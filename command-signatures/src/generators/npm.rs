use warp_completion_metadata::{
    Alias, CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector,
    Suggestion,
};

use crate::generators::git::post_process_branches;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::collections::HashMap;

/// Helper struct used for deserializing a npm/yarn package.json file into the necessary fields
/// needed for generators.
#[derive(Deserialize)]
struct PackageJsonInfo {
    #[serde(default)]
    dependencies: HashMap<String, String>,

    #[serde(default)]
    dev_dependencies: HashMap<String, String>,

    #[serde(default)]
    optional_dependencies: HashMap<String, String>,

    #[serde(default)]
    scripts: HashMap<String, String>,
}

/// Helper struct used for deserializing an npm package.json file. Useful for deserializing a field
/// from a npm package.json file where the schema differs from the yarn package.json file.
#[derive(Deserialize)]
struct NpmPackageJsonInfo {
    #[serde(default)]
    workspaces: Vec<String>,
}

/// Helper struct that matches the output of running `yarn list --depth=0 --json`.
#[derive(Deserialize)]
struct YarnListInfo {
    data: YarnListInfoData,
}

#[derive(Deserialize)]
struct YarnListInfoData {
    trees: Vec<YarnListInfoTree>,
}

#[derive(Deserialize)]
struct YarnListInfoTree {
    name: String,
}

fn get_scripts_generator() -> Generator {
    Generator::script(
        "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let package_info: Result<PackageJsonInfo> = serde_json::from_str(output);

            if let Ok(package_info) = package_info {
                package_info
                    .scripts
                    .into_iter()
                    .map(|(key, value)| Suggestion::with_description(key, value))
                    .collect_unordered_results()
            } else {
                GeneratorResults::default()
            }
        },
    )
}

fn config_list() -> Generator {
    Generator::script("yarn config list", |output| {
        if output.trim().is_empty() {
            return GeneratorResults::default();
        }

        let start_index = output.find('{');
        let end_index = output.find('}');

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            // TODO: fix hacky code that was imported from Fig.
            // reason: JSON parse was not working without double quotes
            let output = &output[start_index..end_index + 1];
            let output = output
                .replace('\'', "\"")
                .replace("/\'/gi", "\"")
                .replace("lastUpdateCheck", "\"lastUpdateCheck\"")
                .replace("registry:", "\"lastUpdateCheck\":");

            let config_object: Result<HashMap<String, Value>> = serde_json::from_str(&output);
            if let Ok(config_object) = config_object {
                return config_object
                    .into_keys()
                    .map(Suggestion::new)
                    .collect_unordered_results();
            }
        }
        GeneratorResults::default()
    })
}

fn get_global_packages_generator() -> Generator {
    Generator::script(r#"cat "$(yarn global dir)/package.json""#, |output| {
        if output.trim().is_empty() {
            return GeneratorResults::default();
        }

        let package_info: Option<PackageJsonInfo> = serde_json::from_str(output).ok();

        let package_info = match package_info {
            None => return GeneratorResults::default(),
            Some(package_info) => package_info,
        };

        package_info
            .dependencies
            .keys()
            .chain(package_info.dev_dependencies.keys())
            .map(Suggestion::new)
            .collect_unordered_results()
    })
}

fn dependencies_generator() -> Generator {
    Generator::script(
        "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let package_info: Result<PackageJsonInfo> = serde_json::from_str(output);
            let package_info = match package_info {
                Err(_) => return GeneratorResults::default(),
                Ok(package_info) => package_info,
            };

            let mut suggestions = package_info
                .dependencies
                .into_keys()
                .map(|key| Suggestion::with_description(key, "dependency"))
                .collect::<Vec<Suggestion>>();

            suggestions.extend(
                package_info
                    .dev_dependencies
                    .into_keys()
                    .map(|key| Suggestion::with_description(key, "devDependency")),
            );

            suggestions.extend(
                package_info
                    .optional_dependencies
                    .into_keys()
                    .map(|key| Suggestion::with_description(key, "optionalDependency")),
            );
            suggestions.into_iter().collect_unordered_results()
        },
    )
}

fn workspace_generator() -> Generator {
    Generator::script("cat $(npm prefix)/package.json", |output| {
        if output.trim().is_empty() {
            return GeneratorResults::default();
        }

        let package_info: Result<NpmPackageJsonInfo> = serde_json::from_str(output);
        let package_info = match package_info {
            Err(_) => return GeneratorResults::default(),
            Ok(package_info) => package_info,
        };

        package_info
            .workspaces
            .into_iter()
            .map(|name| Suggestion::with_description(name, "Workspaces"))
            .collect_unordered_results()
    })
}

fn script_alias_generator() -> Alias {
    Alias::new(
        |_| "cat $(npm prefix)/package.json".to_string(),
        |output, tokens, idx| {
            if output.trim().is_empty() {
                return None;
            }

            let package_info: Result<PackageJsonInfo> = serde_json::from_str(output);

            package_info
                .ok()?
                .scripts
                .into_iter()
                .find(|(key, _)| key == tokens[idx])
                .map(|(_, command)| command)
        },
    )
}

pub fn npm_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("npm")
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("workspace_generator", workspace_generator())
        .add_alias("script_alias", script_alias_generator())
}

pub fn pnpm_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pnpm")
        .add_generator(
            "search_branches",
            Generator::script("git branch --no-color", post_process_branches),
        )
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("dependencies_generator", dependencies_generator())
}

pub fn yarn_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("yarn")
        .add_generator(
            "get_global_packages_generator",
            get_global_packages_generator(),
        )
        .add_generator("config_list", config_list())
        .add_generator("dependencies_generator", dependencies_generator())
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator(
            "all_dependencies_generator",
            Generator::script("yarn list --depth=0 --json", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                let yarn_list_info: YarnListInfo = match serde_json::from_str(output) {
                    Ok(info) => info,
                    Err(e) => {
                        log::warn!(
                            "Failed to deserialize all_dependencies_generator yarn output {:?}",
                            e
                        );
                        return GeneratorResults::default();
                    }
                };

                yarn_list_info
                    .data
                    .trees
                    .into_iter()
                    .filter_map(|tree| {
                        let name = tree.name.rsplit_once('@')?.0;
                        Some(Suggestion::new(name))
                    })
                    .collect_ordered_results()
            }),
        )
        .add_alias("script_alias", script_alias_generator())
}

#[cfg(test)]
mod tests {
    use crate::generators::npm::workspace_generator;
    use warp_completion_metadata::{GeneratorResults, Suggestion};

    #[test]
    pub fn test_workspace_generator() {
        let output = r#"{
              "name": "npm-ts-workspaces-example",
              "private": true,
              "scripts": {
                "clean": "rimraf \"packages/**/lib\" \"packages/**/*.tsbuildinfo\"",
                "compile": "tsc -b tsconfig.build.json",
                "prettier": "prettier \"*.{js,json,yml,md}\" \"packages/**/*\"",
                "format": "npm run prettier -- --write",
                "format:check": "npm run prettier -- --check",
                "lint": "npm run format:check",
                "test": "lerna run test",
                "prepare": "npm run compile"
              },
              "devDependencies": {
                "lerna": "4.0.0",
                "prettier": "2.4.1",
                "rimraf": "3.0.2",
                "typescript": "4.4.4"
              },
              "workspaces": [
                "packages/*"
              ]
        }"#;
        assert_eq!(
            workspace_generator().on_complete(output),
            GeneratorResults {
                suggestions: vec![Suggestion::with_description("packages/*", "Workspaces")],
                is_ordered: false
            }
        );
    }
}
