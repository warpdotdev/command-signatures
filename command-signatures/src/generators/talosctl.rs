//! `talosctl` is the CLI for out-of-band management of Talos Linux Kubernetes nodes.
//! Like `kubectl` and `oc` it is Cobra-based, so dynamic completions (contexts, node
//! services, resource types, …) are delegated to the CLI's own hidden `__complete`
//! command, mirroring the `oc_builtin_completion` approach in `oc.rs`.
use itertools::Itertools;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

fn talosctl_builtin_complete_post_process(output: &str) -> GeneratorResults {
    if output.contains("[Error]") || output.contains("error:") {
        return GeneratorResults::default();
    }
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(':'))
        .map(|line| {
            // Cobra `__complete` emits `value<TAB>description` when a description exists.
            match line.split_once('\t') {
                Some((value, description)) => Suggestion::with_description(value, description),
                None => Suggestion::new(line),
            }
        })
        .collect_ordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    let talosctl_builtin_completion = Generator::command_from_tokens(
        |tokens, has_trailing_whitespace, env_vars| {
            let env_vars_str = env_vars.iter().join(" ");
            let mut generation_command = vec![&env_vars_str, "talosctl", "__complete"]
                .into_iter()
                .chain(
                    // Skip the first token which is just "talosctl"
                    tokens.iter().skip(1).cloned(),
                )
                .collect_vec();
            // The __complete command needs the empty string at the end
            if has_trailing_whitespace {
                generation_command.push("\"\"");
            }
            // Skip the last line since it is metadata, not a completion result.
            CommandBuilder::pipe(
                CommandBuilder::single_command(generation_command.join(" ")),
                CommandBuilder::single_command("sed '$d'"),
            )
        },
        talosctl_builtin_complete_post_process,
    );

    CommandSignatureGenerators::new("talosctl")
        .add_generator("talosctl_builtin_completion", talosctl_builtin_completion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_complete_post_process_plain_values() {
        let output = "dev-cluster\nprod-cluster\nstaging-cluster\n";
        let results = talosctl_builtin_complete_post_process(output);
        assert_eq!(results.suggestions.len(), 3);
        assert_eq!(results.suggestions[0].exact_string, "dev-cluster");
    }

    #[test]
    fn test_builtin_complete_post_process_with_descriptions() {
        let output = "apid\tAPI daemon\netcd\tetcd datastore\n";
        let results = talosctl_builtin_complete_post_process(output);
        assert_eq!(results.suggestions.len(), 2);
        assert_eq!(results.suggestions[0].exact_string, "apid");
        assert_eq!(
            results.suggestions[0].description.as_deref(),
            Some("API daemon")
        );
    }

    #[test]
    fn test_builtin_complete_post_process_filters_errors() {
        let output = "[Debug] [Error] Error while parsing flags\n";
        let results = talosctl_builtin_complete_post_process(output);
        assert!(results.suggestions.is_empty());
    }
}
