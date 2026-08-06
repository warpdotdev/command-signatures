//! `yc` is the Yandex Cloud CLI. Like `kubectl` and `oc`, it is Cobra-based, so its completions are
//! produced by shelling out to the CLI's own hidden completion command (`yc __completeNoDesc`).
//! Driving completions from the installed CLI keeps them in sync with the user's `yc` version
//! instead of hand-maintaining the full command tree in a static spec.
use itertools::Itertools;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Builds the `yc __completeNoDesc ...` command that asks the installed CLI for completions of the
/// command being typed. The final line of Cobra's completion output is a `:<directive>` metadata
/// line, so it is stripped with `sed '$d'`; `CommandBuilder::pipe` also discards the first command's
/// stderr, which is where Cobra writes its "Completion ended with directive" trailer.
fn yc_completion_command(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    env_vars: &[String],
) -> CommandBuilder {
    let env_vars_str = env_vars.iter().join(" ");
    let mut generation_command = vec![&env_vars_str, "yc", "__completeNoDesc"]
        .into_iter()
        .chain(
            // Skip the first token, which is just "yc".
            tokens.iter().skip(1).cloned(),
        )
        .collect_vec();
    // Cobra needs an explicit empty argument to complete a fresh token.
    if has_trailing_whitespace {
        generation_command.push("\"\"");
    }
    CommandBuilder::pipe(
        CommandBuilder::single_command(generation_command.join(" ")),
        CommandBuilder::single_command("sed '$d'"),
    )
}

/// Parses `yc __completeNoDesc` output into suggestions. Cobra emits one completion per line
/// (without descriptions under `__completeNoDesc`), so blank lines, the `:<directive>` metadata
/// line, and the "Completion ended" trailer are dropped, and any error output yields no
/// suggestions. The ordering from the CLI is preserved.
fn yc_completion_post_process(output: &str) -> GeneratorResults {
    if output.contains("ERROR:") || output.contains("error:") {
        return GeneratorResults::default();
    }
    output
        .lines()
        .map(str::trim)
        .filter(|line| {
            !line.is_empty() && !line.starts_with(':') && !line.starts_with("Completion ended")
        })
        .map(Suggestion::new)
        .collect_ordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("yc").add_generator(
        "yc_builtin_completion",
        Generator::command_from_tokens(yc_completion_command, yc_completion_post_process),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp_completion_metadata::Shell;

    #[test]
    fn test_completion_command_completes_fresh_token() {
        let cmd = yc_completion_command(&["yc"], true, &[]);
        assert_eq!(
            cmd.build(Shell::Posix),
            r#" yc __completeNoDesc "" 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_completion_command_completes_nested_subcommand() {
        let cmd = yc_completion_command(&["yc", "compute", "instance"], true, &[]);
        assert_eq!(
            cmd.build(Shell::Posix),
            r#" yc __completeNoDesc compute instance "" 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_completion_command_completes_partial_token() {
        // No trailing whitespace: the last token is a prefix Cobra should match, not a new token.
        let cmd = yc_completion_command(&["yc", "comp"], false, &[]);
        assert_eq!(
            cmd.build(Shell::Posix),
            r#" yc __completeNoDesc comp 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_post_process_filters_metadata_and_keeps_order() {
        let results = yc_completion_post_process(
            "compute\nconfig\n:4\nCompletion ended with directive: ShellCompDirectiveNoFileComp\n",
        );
        assert!(results.is_ordered);
        assert_eq!(
            results
                .suggestions
                .into_iter()
                .map(|suggestion| suggestion.exact_string)
                .collect::<Vec<_>>(),
            vec!["compute", "config"]
        );
    }

    #[test]
    fn test_post_process_returns_nothing_on_error() {
        let results =
            yc_completion_post_process("ERROR: failed to resolve endpoint: connection refused");
        assert!(results.suggestions.is_empty());
    }
}
