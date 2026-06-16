use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use crate::generators::common::{dependencies_generator, get_scripts_generator};
use crate::generators::npm::npm_registry_search_generator;

/// Parses the output of `vp run` (with no arguments) to produce task name
/// suggestions.
///
/// `vp run` prints one task per line in the form `<name>: <command>` where
/// `<name>` is either a bare task name (e.g. `build`) for the current package
/// or a fully-qualified `packageName#taskName` for tasks in other workspace
/// packages. The list is context-aware: when invoked from within a workspace
/// package, that package's tasks appear without a prefix.
///
/// `vp run` exits with code 0 even outside a workspace, writing a diagnostic
/// to stderr (which we suppress via `2>/dev/null`). Lines that don't contain
/// `:` or that start with `error:`/`warning:` are skipped defensively.
fn vp_tasks_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("vp run 2>/dev/null"),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }
            output
                .lines()
                .map(str::trim)
                .filter(|line| {
                    !line.is_empty()
                        && !line.starts_with("error:")
                        && !line.starts_with("warning:")
                        && line.contains(':')
                })
                .filter_map(|line| {
                    let idx = line.find(':')?;
                    let name = line[..idx].trim();
                    let command = line[idx + 1..].trim();
                    if name.is_empty() {
                        return None;
                    }
                    Some(Suggestion::with_description(name, command))
                })
                .collect_unordered_results()
        },
    )
}

/// Returns the set of workspace package names by extracting the unique
/// `pkg#` prefixes from `vp run` output. This avoids depending on any
/// specific underlying package manager (pnpm/yarn/npm).
fn vp_workspace_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("vp run 2>/dev/null"),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }
            let mut names: Vec<String> = Vec::new();
            let mut seen = std::collections::HashSet::new();
            for line in output.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty()
                    || trimmed.starts_with("error:")
                    || trimmed.starts_with("warning:")
                {
                    continue;
                }
                let Some(idx) = trimmed.find(':') else {
                    continue;
                };
                let task_name = trimmed[..idx].trim();
                if let Some(hash_idx) = task_name.find('#') {
                    let pkg = task_name[..hash_idx].trim().to_string();
                    if !pkg.is_empty() && seen.insert(pkg.clone()) {
                        names.push(pkg);
                    }
                }
            }
            names
                .into_iter()
                .map(|name| Suggestion::with_description(name, "workspace package"))
                .collect_unordered_results()
        },
    )
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vp")
        .add_generator("vp_tasks_generator", vp_tasks_generator())
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("dependencies_generator", dependencies_generator())
        .add_generator(
            "workspace_packages_generator",
            vp_workspace_packages_generator(),
        )
        .add_generator("npm_registry_search", npm_registry_search_generator())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vp_tasks_generator() {
        let output = "  dev: vp run website#dev\n  prepare: vp config\n  utils#build: vp pack\n  website#dev: vp dev\n";
        let result = vp_tasks_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 4);
        assert!(!result.is_ordered);

        let names: Vec<&str> = result
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert!(names.contains(&"dev"));
        assert!(names.contains(&"prepare"));
        assert!(names.contains(&"utils#build"));
        assert!(names.contains(&"website#dev"));
    }

    #[test]
    fn test_vp_tasks_generator_empty_output() {
        assert_eq!(
            vp_tasks_generator().on_complete(""),
            GeneratorResults::default()
        );
    }

    #[test]
    fn test_vp_tasks_generator_filters_diagnostics() {
        let output =
            "error: Package not found in workspace\nwarning: something\n  build: vp pack\n";
        let result = vp_tasks_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].exact_string, "build");
    }

    #[test]
    fn test_vp_workspace_packages_generator() {
        let output = "  dev: vp run website#dev\n  utils#build: vp pack\n  utils#test: vp test\n  website#dev: vp dev\n";
        let result = vp_workspace_packages_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 2);
        assert!(!result.is_ordered);

        let names: Vec<&str> = result
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert!(names.contains(&"utils"));
        assert!(names.contains(&"website"));
    }

    #[test]
    fn test_vp_workspace_packages_generator_no_packages() {
        // Only root package tasks (no `#` prefix) → no workspace packages.
        let output = "  dev: vp dev\n  build: vp build\n";
        let result = vp_workspace_packages_generator().on_complete(output);
        assert_eq!(result.suggestions.len(), 0);
    }

    #[test]
    fn test_vp_workspace_packages_generator_empty_output() {
        assert_eq!(
            vp_workspace_packages_generator().on_complete(""),
            GeneratorResults::default()
        );
    }
}
