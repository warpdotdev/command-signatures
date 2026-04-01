use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("sdk")
        .add_generator(
            "sdk_candidates",
            Generator::script(
                CommandBuilder::single_command("cat ~/.sdkman/var/candidates"),
                |output| {
                    output
                        .trim()
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|candidate| {
                            Suggestion::with_description(candidate.trim(), "SDK candidate")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "sdk_installed_candidates",
            Generator::script(
                CommandBuilder::single_command("ls -1 ~/.sdkman/candidates"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|s| !s.is_empty())
                        .map(|candidate| {
                            Suggestion::with_description(candidate.trim(), "Installed candidate")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "sdk_versions",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| {
                    // The candidate name is the token immediately before the version position.
                    // With trailing whitespace, the last completed token is the candidate.
                    // Without trailing whitespace, the second-to-last token is the candidate.
                    let candidate = if has_trailing_whitespace {
                        tokens.last().copied()
                    } else {
                        tokens.get(tokens.len().saturating_sub(2)).copied()
                    };
                    match candidate {
                        Some(candidate) if !candidate.is_empty() && !candidate.starts_with('-') => {
                            CommandBuilder::single_command(format!(
                                "ls -1 ~/.sdkman/candidates/{}/",
                                candidate
                            ))
                        }
                        _ => CommandBuilder::single_command("echo"),
                    }
                },
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|s| !s.is_empty() && *s != "current")
                        .map(|version| {
                            Suggestion::with_description(version.trim(), "Installed version")
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
