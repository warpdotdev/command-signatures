use itertools::Itertools;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref NX_WORKSPACE_TARGETS_COMMAND: CommandBuilder =
        CommandBuilder::single_command("nx graph --file=stdout");
}

/// Parsed output from `nx graph --file=stdout`.
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

fn process_generators(output: &str) -> GeneratorResults {
    output
        .split('\n')
        .filter_map(|line| line.split(' ').next().map(Suggestion::new))
        .collect_unordered_results()
}

/// Parse the output of `nx show projects` (one project name per line) into suggestions.
fn process_project_list(output: &str) -> GeneratorResults {
    output
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Suggestion::new(line.trim()))
        .collect_unordered_results()
}

/// Parse the project graph output into `project:target` suggestions.
fn process_workspace_targets(output: &str) -> GeneratorResults {
    let Ok(parsed_output) = serde_json::from_str::<NXGraphFile>(output) else {
        return GeneratorResults::default();
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
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("nx")
        .add_generator(
            "apps",
            Generator::script(
                CommandBuilder::single_command(
                    "sh -c \"nx show projects --type app 2>/dev/null | grep -v -- '-e2e\\$'\"",
                ),
                process_project_list,
            ),
        )
        .add_generator(
            "e2e_apps",
            Generator::script(
                CommandBuilder::single_command(
                    "sh -c \"nx show projects --type e2e 2>/dev/null || nx show projects --type app 2>/dev/null | grep -- '-e2e\\$'\"",
                ),
                process_project_list,
            ),
        )
        .add_generator(
            "apps_and_libs",
            Generator::script(
                CommandBuilder::single_command("nx show projects"),
                process_project_list,
            ),
        )
        .add_generator(
            "local_schematics",
            Generator::script(
                CommandBuilder::single_command("ls tools/schematics"),
                process_generators,
            ),
        )
        .add_generator(
            "local_generators",
            Generator::script(
                CommandBuilder::single_command("ls tools/generators"),
                process_generators,
            ),
        )
        .add_generator(
            "workspace_targets",
            Generator::script(
                NX_WORKSPACE_TARGETS_COMMAND.clone(),
                process_workspace_targets,
            ),
        )
        .add_generator(
            "installed_plugins",
            Generator::script(CommandBuilder::single_command("nx list"), |output| {
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
    use super::*;
    use warp_completion_metadata::Shell;

    /// Regression test for APP-5384: a future `CommandBuilder` refactor (like the one in PR #166
    /// that silently turned a stdout redirect into a stderr redirect) must not be able to change
    /// the built command string without this test catching it.
    #[test]
    fn test_workspace_targets_command_string() {
        assert_eq!(
            NX_WORKSPACE_TARGETS_COMMAND.build(Shell::Posix),
            "nx graph --file=stdout"
        );
    }

    #[test]
    fn test_process_workspace_targets_parses_projects_and_targets() {
        let output = r#"{
            "graph": {
                "nodes": {
                    "admin-integrations": {
                        "name": "admin-integrations",
                        "type": "app",
                        "data": {
                            "root": "apps/admin-integrations",
                            "targets": {
                                "build": {},
                                "serve": {}
                            }
                        }
                    },
                    "utils": {
                        "name": "utils",
                        "type": "lib",
                        "data": {
                            "root": "libs/utils",
                            "targets": {
                                "test": {}
                            }
                        }
                    }
                },
                "dependencies": {}
            }
        }"#;

        let results = process_workspace_targets(output);
        assert!(!results.is_ordered);

        let mut names: Vec<&str> = results
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        names.sort_unstable();
        assert_eq!(
            names,
            vec![
                "admin-integrations:build",
                "admin-integrations:serve",
                "utils:test"
            ]
        );
        assert!(results
            .suggestions
            .iter()
            .all(|s| s.description.as_deref() == Some("nx target")));
    }

    /// Regression test for the original Dec 2024 bug (PR #166): if a human-readable banner from
    /// `nx` ever precedes the JSON on stdout again, parsing must fail closed (no suggestions),
    /// not panic or silently return garbage.
    #[test]
    fn test_process_workspace_targets_fails_closed_on_banner_text() {
        let output =
            " NX   JSON output created in /tmp/tmp.DrVPzrwmkX.json\n{\"graph\":{\"nodes\":{}}}";
        assert_eq!(
            process_workspace_targets(output),
            GeneratorResults::default()
        );
    }

    #[test]
    fn test_process_workspace_targets_empty_graph() {
        let output = r#"{"graph": {"nodes": {}}}"#;
        let results = process_workspace_targets(output);
        assert!(results.suggestions.is_empty());
    }
}
