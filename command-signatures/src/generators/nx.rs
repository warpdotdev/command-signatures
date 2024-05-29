use itertools::Itertools;
use serde_json::{Result, Value};
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NXGraphFile {
    graph: NXGraph,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
struct NXGraph {
    nodes: HashMap<String, NXNode>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NXNode {
    name: String,
    data: NXData,
}

#[derive(Debug, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct NXData {
    targets: HashMap<String, Value>,
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
            "workspace_targets",
            Generator::script("nx graph --file stdout", |output| {
                let Ok(parsed_output) = serde_json::from_str::<NXGraphFile>(output) else {
                    return GeneratorResults::default()
                };

                let suggestions = parsed_output
                    .graph
                    .nodes
                    .into_values()
                    .flat_map(|node| {
                        node.data.targets.into_keys().map(move |target| {
                            let name = format!("{}:{target}", node.name);
                            Suggestion::with_description(name, "nx target")
                        })
                    })
                    .unique();
                GeneratorResults {
                    suggestions: suggestions.collect(),
                    is_ordered: false,
                }
            }),
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

#[cfg(test)]
mod tests {
    use crate::generators::nx::{NXGraph, NXGraphFile};

    #[test]
    fn test_deserialize_run_targets() {
        let content = r#"
        {
  "graph": {
    "nodes": {
      "e2e": {
        "name": "e2e",
        "type": "e2e",
        "data": {
          "root": "e2e",
          "targets": {
            "lint": {
              "cache": true,
              "options": {
                "cwd": "e2e",
                "command": "eslint ."
              },
              "inputs": [
                "default",
                "^default",
                "{workspaceRoot}/.eslintrc.json",
                "{projectRoot}/.eslintrc.json",
                "{workspaceRoot}/tools/eslint-rules/**/*",
                {
                  "externalDependencies": [
                    "eslint"
                  ]
                }
              ],
              "outputs": [
                "{options.outputFile}"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "e2e": {
              "options": {
                "cwd": "e2e",
                "command": "playwright test"
              },
              "metadata": {
                "technologies": [
                  "playwright"
                ],
                "description": "Runs Playwright Tests"
              },
              "cache": true,
              "inputs": [
                "default",
                "^production"
              ],
              "outputs": [
                "{workspaceRoot}/dist/.playwright/e2e/playwright-report",
                "{workspaceRoot}/dist/.playwright/e2e/test-output"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "e2e-ci--src/example.spec.ts": {
              "options": {
                "cwd": "e2e",
                "command": "playwright test src/example.spec.ts"
              },
              "metadata": {
                "technologies": [
                  "playwright"
                ],
                "description": "Runs Playwright Tests in src/example.spec.ts in CI"
              },
              "cache": true,
              "inputs": [
                "default",
                "^production"
              ],
              "outputs": [
                "{workspaceRoot}/dist/.playwright/e2e/playwright-report",
                "{workspaceRoot}/dist/.playwright/e2e/test-output"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "e2e-ci": {
              "executor": "nx:noop",
              "cache": true,
              "inputs": [
                "default",
                "^production"
              ],
              "outputs": [
                "{workspaceRoot}/dist/.playwright/e2e/playwright-report",
                "{workspaceRoot}/dist/.playwright/e2e/test-output"
              ],
              "dependsOn": [
                {
                  "target": "e2e-ci--src/example.spec.ts",
                  "projects": "self",
                  "params": "forward"
                }
              ],
              "metadata": {
                "technologies": [
                  "playwright"
                ],
                "description": "Runs Playwright Tests in CI"
              },
              "options": {},
              "configurations": {}
            }
          },
          "metadata": {
            "targetGroups": {
              "E2E (CI)": [
                "e2e-ci--src/example.spec.ts",
                "e2e-ci"
              ]
            }
          },
          "name": "e2e",
          "$schema": "../node_modules/nx/schemas/project-schema.json",
          "projectType": "application",
          "sourceRoot": "e2e/src",
          "implicitDependencies": [
            "myreactapp"
          ],
          "tags": []
        }
      },
      "myreactapp": {
        "name": "myreactapp",
        "type": "app",
        "data": {
          "root": ".",
          "includedScripts": [],
          "name": "myreactapp",
          "targets": {
            "build": {
              "options": {
                "cwd": ".",
                "command": "vite build"
              },
              "cache": true,
              "dependsOn": [
                "^build"
              ],
              "inputs": [
                "production",
                "^production",
                {
                  "externalDependencies": [
                    "vite"
                  ]
                }
              ],
              "outputs": [
                "{projectRoot}/dist/myreactapp"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "serve": {
              "options": {
                "cwd": ".",
                "command": "vite serve"
              },
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "preview": {
              "options": {
                "cwd": ".",
                "command": "vite preview"
              },
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "serve-static": {
              "executor": "@nx/web:file-server",
              "options": {
                "buildTarget": "build",
                "spa": true
              },
              "configurations": {}
            },
            "test": {
              "options": {
                "cwd": ".",
                "watch": false,
                "command": "vitest"
              },
              "cache": true,
              "inputs": [
                "default",
                "^production",
                {
                  "externalDependencies": [
                    "vitest"
                  ]
                }
              ],
              "outputs": [
                "{projectRoot}/coverage/myreactapp"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            },
            "lint": {
              "cache": true,
              "options": {
                "cwd": ".",
                "command": "eslint ./src"
              },
              "inputs": [
                "default",
                "^default",
                "{projectRoot}/eslintrc.json",
                "{projectRoot}/.eslintignore",
                "{workspaceRoot}/tools/eslint-rules/**/*",
                {
                  "externalDependencies": [
                    "eslint"
                  ]
                }
              ],
              "outputs": [
                "{options.outputFile}"
              ],
              "executor": "nx:run-commands",
              "configurations": {}
            }
          },
          "sourceRoot": "./src",
          "projectType": "application",
          "$schema": "node_modules/nx/schemas/project-schema.json",
          "tags": [],
          "implicitDependencies": []
        }
      }
    },
    "dependencies": {
      "e2e": [
        {
          "source": "e2e",
          "target": "myreactapp",
          "type": "implicit"
        }
      ],
      "myreactapp": []
    }
  }
}
"#;
        let graph: NXGraphFile = serde_json::from_str(content).unwrap();
        println!("graph is {graph:?}");
        panic!("foo");
    }
}
