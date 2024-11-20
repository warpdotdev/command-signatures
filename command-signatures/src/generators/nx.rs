use itertools::Itertools;
use lazy_static::lazy_static;
use serde_json::{Result, Value};
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    /// Command that computes the `nx` workspace targets by executing `nx graph --file`.
    /// `nx graph` supports printing to stdout, but older versions of `nx` (before 19.20) had a bug
    /// where the output of `nx graph` could be truncated when printing to stdout (see https://github.com/nrwl/nx/issues/18689
    /// for more details).
    /// The workaround here is to write the output to a tmpfile and then `cat` that tmpfile. We execute
    /// this within a sh shell to ensure we are running in an environment where we can run POSIX-shell
    /// compliant commands to generate the output, even if the user is running a non-POSIX compliant
    /// shell (such as fish).
    static ref NX_WORKSPACE_TARGETS_COMMAND: CommandBuilder = CommandBuilder::and(
        CommandBuilder::single_command_and_ignore_stderr(
            "sh -c 'temp=$(mktemp -u).json && nx graph --file $temp"
        ),
        CommandBuilder::single_command("cat $temp && rm $temp'")
    );
}

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
            Generator::script(
                CommandBuilder::single_command("cat workspace.json"),
                |output| {
                    process_workspace_json(output, |(name, project)| {
                        project.project_type == "application" && !name.ends_with("-e2e")
                    })
                },
            ),
        )
        .add_generator(
            "e2e_apps",
            Generator::script(
                CommandBuilder::single_command("cat workspace.json"),
                |output| {
                    process_workspace_json(output, |(name, project)| {
                        project.project_type == "application" && name.ends_with("-e2e")
                    })
                },
            ),
        )
        .add_generator(
            "apps_and_libs",
            Generator::script(
                CommandBuilder::single_command("cat workspace.json"),
                |output| process_workspace_json(output, |_| true),
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
            Generator::script(NX_WORKSPACE_TARGETS_COMMAND.clone(), |output| {
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
            }),
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
