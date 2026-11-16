use std::collections::BTreeMap;

use serde::Deserialize;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use super::common::dependencies_generator;
use super::npm::npm_registry_search_generator;

/// Subset of the task graph printed by `vp run --json`.
#[derive(Deserialize)]
struct TaskGraph {
    #[serde(rename = "workspaceRoot")]
    workspace_root: String,
    #[serde(default)]
    packages: BTreeMap<String, GraphPackage>,
    #[serde(default)]
    tasks: BTreeMap<String, GraphTask>,
}

#[derive(Deserialize)]
struct GraphPackage {
    #[serde(default)]
    name: String,
}

#[derive(Deserialize)]
struct GraphTask {
    /// Workspace-relative directory of the package that owns the task (`.` for the root).
    package: String,
    name: String,
    #[serde(default)]
    command: String,
}

/// Prints the physical working directory followed by the task graph of the workspace.
///
/// `vp run --json` prints every package and task in the workspace, independent of the working
/// directory, so the directory is printed first to let the callback tell which package the user
/// is currently in.
const TASK_GRAPH_COMMAND: &str = "pwd -P; vp run --json 2>/dev/null";

/// Splits the output of [`TASK_GRAPH_COMMAND`] into the working directory and the task graph.
fn parse_task_graph(output: &str) -> Option<(&str, TaskGraph)> {
    let (cwd, rest) = output.split_once('\n')?;
    let json = &rest[rest.find('{')?..];
    let graph = serde_json::from_str(json).ok()?;
    Some((cwd.trim(), graph))
}

/// Absolute directory of a workspace-relative package path.
fn package_dir(workspace_root: &str, package_path: &str) -> String {
    let root = workspace_root.trim_end_matches('/');
    if package_path == "." {
        root.to_string()
    } else {
        format!("{root}/{package_path}")
    }
}

/// Returns the workspace-relative path of the innermost package that contains `cwd`.
fn current_package<'a>(cwd: &str, graph: &'a TaskGraph) -> Option<&'a str> {
    graph
        .packages
        .keys()
        .filter(|path| {
            let dir = package_dir(&graph.workspace_root, path);
            cwd == dir || cwd.starts_with(&format!("{dir}/"))
        })
        .max_by_key(|path| package_dir(&graph.workspace_root, path).len())
        .map(String::as_str)
}

/// Suggests tasks for `vp run`.
///
/// Tasks of the package the user is in are suggested by their bare name. Every task in the
/// workspace is also suggested in the `package#task` form that `vp run` accepts.
fn tasks_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(TASK_GRAPH_COMMAND),
        |output| {
            let Some((cwd, graph)) = parse_task_graph(output) else {
                return GeneratorResults::default();
            };
            let current = current_package(cwd, &graph);

            let mut suggestions = Vec::new();
            for task in graph.tasks.values() {
                if current == Some(task.package.as_str()) {
                    suggestions.push(Suggestion::with_description(
                        task.name.clone(),
                        task.command.clone(),
                    ));
                }
                let package_name = graph
                    .packages
                    .get(&task.package)
                    .map(|package| package.name.as_str())
                    .unwrap_or_default();
                if !package_name.is_empty() {
                    suggestions.push(Suggestion::with_description(
                        format!("{package_name}#{}", task.name),
                        task.command.clone(),
                    ));
                }
            }
            suggestions.into_iter().collect_unordered_results()
        },
    )
}

/// Suggests the names of the packages in the workspace, for `--filter`.
fn workspace_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(TASK_GRAPH_COMMAND),
        |output| {
            let Some((_, graph)) = parse_task_graph(output) else {
                return GeneratorResults::default();
            };

            graph
                .packages
                .into_iter()
                .filter(|(_, package)| !package.name.is_empty())
                .map(|(path, package)| {
                    let description = if path == "." {
                        "workspace root".to_string()
                    } else {
                        path
                    };
                    Suggestion::with_description(package.name, description)
                })
                .collect_unordered_results()
        },
    )
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vp")
        .add_generator("vp_tasks_generator", tasks_generator())
        .add_generator(
            "vp_workspace_packages_generator",
            workspace_packages_generator(),
        )
        .add_generator("dependencies_generator", dependencies_generator())
        .add_generator("npm_registry_search", npm_registry_search_generator())
}

#[cfg(test)]
mod tests {
    use super::{tasks_generator, workspace_packages_generator};
    use warp_completion_metadata::{GeneratorResults, Suggestion};

    /// Output of `pwd -P; vp run --json` for a workspace with a root package and two packages.
    fn output(cwd: &str) -> String {
        format!(
            r#"{cwd}
{{"$schema":"file:///tmp/task-graph.json","version":1,"workspaceRoot":"/work/mono","packages":{{".":{{"name":"root"}},"packages/a":{{"name":"@m/a"}},"packages/b":{{"name":"@m/b"}}}},"tasks":{{"lint":{{"package":".","name":"lint","command":"echo lint"}},"packages/a#build":{{"package":"packages/a","name":"build","command":"echo a"}},"packages/a#test":{{"package":"packages/a","name":"test","command":"echo t"}},"packages/b#build":{{"package":"packages/b","name":"build","command":"echo b"}}}}}}"#
        )
    }

    fn sorted(results: GeneratorResults) -> Vec<Suggestion> {
        let mut suggestions = results.suggestions;
        suggestions.sort_by(|a, b| a.exact_string.cmp(&b.exact_string));
        suggestions
    }

    #[test]
    fn test_tasks_generator_in_package_suggests_bare_and_qualified_names() {
        let results = tasks_generator().on_complete(&output("/work/mono/packages/a/src"));
        assert_eq!(
            sorted(results),
            vec![
                Suggestion::with_description("@m/a#build", "echo a"),
                Suggestion::with_description("@m/a#test", "echo t"),
                Suggestion::with_description("@m/b#build", "echo b"),
                Suggestion::with_description("build", "echo a"),
                Suggestion::with_description("root#lint", "echo lint"),
                Suggestion::with_description("test", "echo t"),
            ]
        );
    }

    #[test]
    fn test_tasks_generator_at_workspace_root_suggests_root_tasks_bare() {
        let results = tasks_generator().on_complete(&output("/work/mono"));
        assert_eq!(
            sorted(results),
            vec![
                Suggestion::with_description("@m/a#build", "echo a"),
                Suggestion::with_description("@m/a#test", "echo t"),
                Suggestion::with_description("@m/b#build", "echo b"),
                Suggestion::with_description("lint", "echo lint"),
                Suggestion::with_description("root#lint", "echo lint"),
            ]
        );
    }

    #[test]
    fn test_tasks_generator_does_not_confuse_sibling_directories_with_prefix() {
        // `/work/mono/packages/a-extra` shares a string prefix with `/work/mono/packages/a`.
        let results = tasks_generator().on_complete(&output("/work/mono/packages/a-extra"));
        assert!(results
            .suggestions
            .iter()
            .all(|suggestion| suggestion.exact_string != "build"
                && suggestion.exact_string != "test"));
    }

    #[test]
    fn test_tasks_generator_empty_and_invalid_output() {
        assert_eq!(
            tasks_generator().on_complete(""),
            GeneratorResults::default()
        );
        assert_eq!(
            tasks_generator().on_complete("/work/mono\nnot json"),
            GeneratorResults::default()
        );
        assert_eq!(
            tasks_generator().on_complete("/work/mono\n"),
            GeneratorResults::default()
        );
    }

    #[test]
    fn test_workspace_packages_generator() {
        let results = workspace_packages_generator().on_complete(&output("/work/mono"));
        assert_eq!(
            sorted(results),
            vec![
                Suggestion::with_description("@m/a", "packages/a"),
                Suggestion::with_description("@m/b", "packages/b"),
                Suggestion::with_description("root", "workspace root"),
            ]
        );
    }

    #[test]
    fn test_workspace_packages_generator_empty_output() {
        assert_eq!(
            workspace_packages_generator().on_complete(""),
            GeneratorResults::default()
        );
    }
}
