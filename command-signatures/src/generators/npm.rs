use warp_completion_metadata::{
    Alias, CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use crate::generators::common::{dependencies_generator, get_scripts_generator, PackageJsonInfo};
use crate::generators::git::post_process_branches;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::collections::HashMap;

/// Response from the npm registry search API (`/-/v1/search`).
#[derive(Deserialize)]
struct NpmSearchResponse {
    #[serde(default)]
    objects: Vec<NpmSearchObject>,
}

#[derive(Deserialize)]
struct NpmSearchObject {
    package: NpmSearchPackage,
}

#[derive(Deserialize)]
struct NpmSearchPackage {
    name: String,
    #[serde(default)]
    description: Option<String>,
}

/// Helper struct used for deserializing an npm package.json file.
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

/// Returns the list of executables located within the `node_modules` directory.
fn executables_within_node_modules() -> Generator {
    Generator::script(CommandBuilder::single_command(
        "until [[ -d node_modules/ ]] || [[ $PWD = '/' ]]; do cd ..; done; ls -1 node_modules/.bin/"),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            output.lines().map(|item| {
                Suggestion::with_description(item, "Binary from a yarn workspace dependency")
            }).collect_unordered_results()
        },
    )
}

fn config_list() -> Generator {
    Generator::script(
        CommandBuilder::single_command("yarn config list"),
        |output| {
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
        },
    )
}

fn get_global_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(r#"cat "$(yarn global dir)/package.json""#),
        |output| {
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
        },
    )
}

fn workspace_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("cat $(npm prefix)/package.json"),
        |output| {
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
        },
    )
}

/// Returns actual workspace names for yarn projects by parsing the output of
/// `yarn workspaces info`. The output wraps a JSON object in extra text
/// (version header and "Done" footer), so we extract the JSON between the first
/// `{` and last `}` and use the top-level keys as workspace names.
fn yarn_workspace_names_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("yarn workspaces info 2>/dev/null"),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let start_index = output.find('{');
            let end_index = output.rfind('}');

            if let (Some(start), Some(end)) = (start_index, end_index) {
                let json_str = &output[start..=end];
                let workspaces: std::result::Result<HashMap<String, Value>, _> =
                    serde_json::from_str(json_str);

                if let Ok(workspaces) = workspaces {
                    return workspaces
                        .into_keys()
                        .map(|name| Suggestion::with_description(name, "Workspace"))
                        .collect_unordered_results();
                }
            }

            GeneratorResults::default()
        },
    )
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

/// Searches the npm registry for packages matching the current input.
/// Uses the official npm registry search endpoint.
fn npm_registry_search_generator() -> Generator {
    Generator::command_from_tokens(
        |tokens, trailing_whitespace, _| {
            // When there is trailing whitespace the user has not started typing
            // the next package name yet, so there is nothing to search for.
            let query = if trailing_whitespace {
                ""
            } else {
                tokens.last().copied().unwrap_or("")
            };

            if query.is_empty() {
                // No-op: produce empty output so the callback returns no suggestions.
                CommandBuilder::single_command("printf ''")
            } else {
                // Sanitize the query to prevent shell injection: keep only
                // characters that are valid in npm package names
                // (alphanumeric, hyphen, dot, underscore, @, /).
                let safe_query: String = query
                    .chars()
                    .filter(|c| c.is_alphanumeric() || "-.@/_".contains(*c))
                    .collect();

                if safe_query.is_empty() {
                    CommandBuilder::single_command("printf ''")
                } else {
                    CommandBuilder::single_command(format!(
                        "curl -sf 'https://registry.npmjs.org/-/v1/search?text={}&size=20'",
                        safe_query
                    ))
                }
            }
        },
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let response: std::result::Result<NpmSearchResponse, _> = serde_json::from_str(output);

            match response {
                Ok(response) => response
                    .objects
                    .into_iter()
                    .map(|obj| {
                        Suggestion::with_description(
                            obj.package.name,
                            obj.package.description.unwrap_or_default(),
                        )
                    })
                    .collect_ordered_results(),
                Err(_) => GeneratorResults::default(),
            }
        },
    )
}

pub fn npm_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("npm")
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("workspace_generator", workspace_generator())
        .add_generator("npm_registry_search", npm_registry_search_generator())
        .add_alias("script_alias", script_alias_generator())
}

pub fn pnpm_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pnpm")
        .add_generator(
            "search_branches",
            Generator::script(
                CommandBuilder::single_command("git branch --no-color"),
                post_process_branches,
            ),
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
            "workspace_names_generator",
            yarn_workspace_names_generator(),
        )
        .add_generator(
            "all_dependencies_generator",
            Generator::script(
                CommandBuilder::single_command("yarn list --depth=0 --json"),
                |output| {
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
                },
            ),
        )
        .add_generator(
            "executables_within_node_modules",
            executables_within_node_modules(),
        )
        .add_alias("script_alias", script_alias_generator())
}

#[cfg(test)]
mod tests {
    use crate::generators::npm::{
        npm_registry_search_generator, workspace_generator, yarn_workspace_names_generator,
    };
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

    #[test]
    pub fn test_yarn_workspace_names_generator() {
        // Simulates the output of `yarn workspaces info` in yarn v1
        let output = r#"yarn workspaces v1.22.19
{
  "@myorg/package-a": {
    "location": "packages/package-a",
    "workspaceDependencies": [],
    "mismatchedWorkspaceDependencies": []
  },
  "@myorg/package-b": {
    "location": "packages/package-b",
    "workspaceDependencies": ["@myorg/package-a"],
    "mismatchedWorkspaceDependencies": []
  }
}
Done in 0.03s."#;
        let result = yarn_workspace_names_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 2);
        assert!(!result.is_ordered);

        let names: Vec<&str> = result
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert!(names.contains(&"@myorg/package-a"));
        assert!(names.contains(&"@myorg/package-b"));
    }

    #[test]
    pub fn test_yarn_workspace_names_generator_empty_output() {
        assert_eq!(
            yarn_workspace_names_generator().on_complete(""),
            GeneratorResults::default()
        );
    }

    #[test]
    pub fn test_npm_registry_search_generator() {
        let output = r#"{
            "objects": [
                {
                    "package": {
                        "name": "express",
                        "description": "Fast, unopinionated, minimalist web framework"
                    }
                },
                {
                    "package": {
                        "name": "express-validator",
                        "description": "Express middleware for validator"
                    }
                }
            ]
        }"#;
        let result = npm_registry_search_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 2);
        assert!(result.is_ordered);
        assert_eq!(result.suggestions[0].exact_string, "express");
        assert_eq!(
            result.suggestions[0].description.as_deref(),
            Some("Fast, unopinionated, minimalist web framework")
        );
        assert_eq!(result.suggestions[1].exact_string, "express-validator");
    }

    #[test]
    pub fn test_npm_registry_search_generator_empty_output() {
        assert_eq!(
            npm_registry_search_generator().on_complete(""),
            GeneratorResults::default()
        );
    }

    #[test]
    pub fn test_npm_registry_search_generator_no_description() {
        let output = r#"{
            "objects": [
                {
                    "package": {
                        "name": "my-package"
                    }
                }
            ]
        }"#;
        let result = npm_registry_search_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].exact_string, "my-package");
        assert_eq!(result.suggestions[0].description.as_deref(), Some(""));
    }
}
