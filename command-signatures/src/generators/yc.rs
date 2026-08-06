//! `yc` is the Yandex Cloud CLI. Like `kubectl` and `oc`, it is Cobra-based, so its completions are
//! produced by shelling out to the CLI's own hidden completion command (`yc __complete`). Driving
//! completions from the installed CLI keeps them in sync with the user's `yc` version instead of
//! hand-maintaining the full command tree in a static spec.
use itertools::Itertools;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Builds the `yc __complete ...` command that asks the installed CLI for completions of the command
/// being typed. The final line of Cobra's completion output is a `:<directive>` metadata line, so it
/// is stripped with `sed '$d'`; `CommandBuilder::pipe` also discards the first command's stderr,
/// which is where Cobra writes its "Completion ended with directive" trailer.
fn yc_completion_command(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    env_vars: &[String],
) -> CommandBuilder {
    let env_vars_str = env_vars.iter().join(" ");
    let mut generation_command = vec![&env_vars_str, "yc", "__complete"]
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

/// Parses `yc __complete` output into suggestions. Cobra emits one completion per line as
/// `value<TAB>description` (the description is omitted when empty), so each line is split on the
/// first tab to carry the description through. Blank lines, the `:<directive>` metadata line, and
/// the "Completion ended" trailer are dropped, and any error output yields no suggestions. The
/// ordering from the CLI is preserved.
fn yc_completion_post_process(output: &str) -> GeneratorResults {
    if output.contains("ERROR:") || output.contains("error:") {
        return GeneratorResults::default();
    }
    output
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with(':')
                && !trimmed.starts_with("Completion ended")
        })
        .map(|line| match line.split_once('\t') {
            Some((value, description)) if !description.trim().is_empty() => {
                Suggestion::with_description(value.trim(), description.trim())
            }
            Some((value, _)) => Suggestion::new(value.trim()),
            None => Suggestion::new(line.trim()),
        })
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
            r#" yc __complete "" 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_completion_command_completes_nested_subcommand() {
        let cmd = yc_completion_command(&["yc", "compute", "instance"], true, &[]);
        assert_eq!(
            cmd.build(Shell::Posix),
            r#" yc __complete compute instance "" 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_completion_command_completes_partial_token() {
        // No trailing whitespace: the last token is a prefix Cobra should match, not a new token.
        let cmd = yc_completion_command(&["yc", "comp"], false, &[]);
        assert_eq!(
            cmd.build(Shell::Posix),
            r#" yc __complete comp 2>/dev/null | sed '$d'"#
        );
    }

    #[test]
    fn test_post_process_parses_descriptions_and_filters_metadata() {
        let results = yc_completion_post_process(
            "compute\tManage Compute Cloud resources\nconfig\tManage CLI config\n:4\nCompletion ended with directive: ShellCompDirectiveNoFileComp\n",
        );
        assert!(results.is_ordered);
        assert_eq!(
            results
                .suggestions
                .into_iter()
                .map(|suggestion| (suggestion.exact_string, suggestion.description))
                .collect::<Vec<_>>(),
            vec![
                (
                    "compute".to_owned(),
                    Some("Manage Compute Cloud resources".to_owned())
                ),
                ("config".to_owned(), Some("Manage CLI config".to_owned())),
            ]
        );
    }

    #[test]
    fn test_post_process_handles_missing_description() {
        // A line with no tab, and a line with a tab but an empty description, both yield a
        // description-less suggestion.
        let results = yc_completion_post_process("vpc\ndns\t\n");
        assert_eq!(
            results
                .suggestions
                .into_iter()
                .map(|suggestion| (suggestion.exact_string, suggestion.description))
                .collect::<Vec<_>>(),
            vec![("vpc".to_owned(), None), ("dns".to_owned(), None)]
        );
    }

    #[test]
    fn test_post_process_returns_nothing_on_error() {
        let results =
            yc_completion_post_process("ERROR: failed to resolve endpoint: connection refused");
        assert!(results.suggestions.is_empty());
    }
}
