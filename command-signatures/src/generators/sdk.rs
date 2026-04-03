use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

/// Returns the SDKMAN directory reference using the SDKMAN_DIR env var with a fallback.
fn sdkman_dir() -> &'static str {
    "${SDKMAN_DIR:-$HOME/.sdkman}"
}

/// Extracts the candidate name from tokens.
///
/// When `has_trailing_whitespace` is true, the candidate is the last token
/// (the user just finished typing the candidate and pressed space).
/// Otherwise, the candidate is the second-to-last token (the user is typing
/// the version prefix).
fn candidate_from_tokens<'a>(tokens: &'a [&str], has_trailing_whitespace: bool) -> Option<&'a str> {
    if has_trailing_whitespace {
        tokens.last().copied()
    } else {
        tokens.get(tokens.len().saturating_sub(2)).copied()
    }
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("sdk")
        .add_generator(
            "candidates",
            Generator::script(
                CommandBuilder::single_command(format!(
                    "cat {}/var/candidates | tr ',' '\\n'",
                    sdkman_dir()
                )),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "SDK candidate"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "installed_versions",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| {
                    match candidate_from_tokens(tokens, has_trailing_whitespace) {
                        Some(candidate) => CommandBuilder::pipe(
                            CommandBuilder::single_command(format!(
                                "ls {}/candidates/{}/",
                                sdkman_dir(),
                                candidate
                            )),
                            CommandBuilder::single_command("grep -v current"),
                        ),
                        None => CommandBuilder::single_command(""),
                    }
                },
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| {
                            Suggestion::with_description(line.trim(), "Installed version")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "available_versions",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| {
                    match candidate_from_tokens(tokens, has_trailing_whitespace) {
                        Some(candidate) => CommandBuilder::single_command(format!(
                            "curl -s https://api.sdkman.io/2/candidates/{}/$(cat {}/var/platform)/versions/all | tr ',' '\\n'",
                            candidate,
                            sdkman_dir()
                        )),
                        None => CommandBuilder::single_command(""),
                    }
                },
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| {
                            Suggestion::with_description(line.trim(), "Available version")
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
