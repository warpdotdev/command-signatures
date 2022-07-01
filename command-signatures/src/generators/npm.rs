use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

use serde_json::{Result, Value};
use std::collections::HashMap;

/// Helper struct used for deserializing a npm/yarn package.json file into the necessary fields
/// needed for generators.
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
struct NpmPackageJsonInfo {
    #[serde(default)]
    workspaces: Vec<String>,
}

fn get_scripts_generator() -> Generator {
    Generator::new(
        "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        |output| {
            if output.trim().is_empty() {
                return vec![];
            }

            let package_info: Result<PackageJsonInfo> = serde_json::from_str(output);

            if let Ok(package_info) = package_info {
                package_info
                    .scripts
                    .into_iter()
                    .map(|(key, value)| Suggestion::with_description(key, value))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        },
    )
}

fn config_list() -> Generator {
    Generator::new("yarn config list", |output| {
        if output.trim().is_empty() {
            return vec![];
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
                    .into_iter()
                    .map(|(key, _)| Suggestion::new(key))
                    .collect::<Vec<_>>();
            }
        }
        vec![]
    })
}

fn get_global_packages_generator() -> Generator {
    Generator::new(r#"cat "$(yarn global dir)/package.json""#, |output| {
        if output.trim().is_empty() {
            return vec![];
        }

        let package_info: Option<PackageJsonInfo> = serde_json::from_str(output).ok();

        let package_info = match package_info {
            None => return vec![],
            Some(package_info) => package_info,
        };

        package_info
            .dependencies
            .keys()
            .chain(package_info.dev_dependencies.keys())
            .map(Suggestion::new)
            .collect()
    })
}

fn dependencies_generator() -> Generator {
    Generator::new(
        "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        |output| {
            if output.trim().is_empty() {
                return vec![];
            }

            let package_info: Result<PackageJsonInfo> = serde_json::from_str(output);
            let package_info = match package_info {
                Err(_) => return vec![],
                Ok(package_info) => package_info,
            };

            let mut suggestions = package_info
                .dependencies
                .into_iter()
                .map(|(key, _)| Suggestion::with_description(key, "dependency"))
                .collect::<Vec<_>>();

            suggestions.extend(
                package_info
                    .dev_dependencies
                    .into_iter()
                    .map(|(key, _)| Suggestion::with_description(key, "devDependency")),
            );

            suggestions.extend(
                package_info
                    .optional_dependencies
                    .into_iter()
                    .map(|(key, _)| Suggestion::with_description(key, "optionalDependency")),
            );
            suggestions
        },
    )
}

fn workspace_generator() -> Generator {
    Generator::new("cat package.json", |output| {
        if output.trim().is_empty() {
            return vec![];
        }

        let package_info: Result<NpmPackageJsonInfo> = serde_json::from_str(output);
        let package_info = match package_info {
            Err(_) => return vec![],
            Ok(package_info) => package_info,
        };

        package_info
            .workspaces
            .into_iter()
            .map(|name| Suggestion::with_description(name, "Workspaces"))
            .collect()
    })
}

pub fn npm_generators() -> CommandGenerators {
    CommandGenerators::new("npm")
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("workspace_generator", workspace_generator())
}

pub fn yarn_generators() -> CommandGenerators {
    CommandGenerators::new("yarn")
        .add_generator(
            "get_global_packages_generator",
            get_global_packages_generator(),
        )
        .add_generator("config_list", config_list())
        .add_generator("dependencies_generator", dependencies_generator())
        .add_generator("get_scripts_generator", get_scripts_generator())
}

#[cfg(test)]
mod tests {
    use crate::generators::npm::workspace_generator;
    use warp_completion_metadata::Suggestion;

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
            vec![Suggestion::with_description("packages/*", "Workspaces")]
        );
    }
}
