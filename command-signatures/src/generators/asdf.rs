use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

/// Extracts the plugin name from the token list.
///
/// When there is trailing whitespace the last token is the plugin name (the user is completing
/// a new argument). Without trailing whitespace the last token is a partial version string and
/// the plugin name is the token before it.
fn plugin_from_tokens<'a>(tokens: &'a [&str], has_trailing_whitespace: bool) -> Option<&'a str> {
    if has_trailing_whitespace {
        tokens.last().copied()
    } else {
        tokens
            .len()
            .checked_sub(2)
            .and_then(|i| tokens.get(i))
            .copied()
    }
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("asdf")
        .add_generator(
            "available_plugins",
            Generator::script(
                CommandBuilder::single_command("asdf plugin list all"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let name = line.split_whitespace().next()?;
                            if name.is_empty() {
                                None
                            } else {
                                Some(Suggestion::with_description(name, "Plugin"))
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "installed_plugins",
            Generator::script(
                CommandBuilder::single_command("asdf plugin list"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Installed plugin"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "installed_versions",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| match plugin_from_tokens(
                    tokens,
                    has_trailing_whitespace,
                ) {
                    Some(plugin) => CommandBuilder::single_command(format!("asdf list {plugin}")),
                    None => CommandBuilder::single_command(""),
                },
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| {
                            let version = line.trim().trim_start_matches('*').trim();
                            Suggestion::with_description(version, "Installed version")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "all_versions",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| match plugin_from_tokens(
                    tokens,
                    has_trailing_whitespace,
                ) {
                    Some(plugin) => {
                        CommandBuilder::single_command(format!("asdf list all {plugin}"))
                    }
                    None => CommandBuilder::single_command(""),
                },
                |output| {
                    output
                        .trim()
                        .lines()
                        .rev()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Version"))
                        .collect_ordered_results()
                },
            ),
        )
        .add_generator(
            "shims",
            Generator::script(
                CommandBuilder::single_command("ls -1 \"${ASDF_DATA_DIR:-$HOME/.asdf}/shims\""),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Shim"))
                        .collect_unordered_results()
                },
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_plugins_post_process() {
        let gen = generator();
        let generator = gen
            .generators()
            .iter()
            .find(|(name, _)| name.0 == "available_plugins")
            .unwrap()
            .1;

        let output = "nodejs        https://github.com/asdf-vm/asdf-nodejs.git\npython        https://github.com/asdf-community/asdf-python.git\n";
        let results = generator.on_complete(output);
        assert_eq!(results.suggestions.len(), 2);
        assert_eq!(results.suggestions[0].exact_string, "nodejs");
        assert_eq!(results.suggestions[1].exact_string, "python");
    }

    #[test]
    fn test_installed_plugins_post_process() {
        let gen = generator();
        let generator = gen
            .generators()
            .iter()
            .find(|(name, _)| name.0 == "installed_plugins")
            .unwrap()
            .1;

        let output = "nodejs\npython\n";
        let results = generator.on_complete(output);
        assert_eq!(results.suggestions.len(), 2);
        assert_eq!(results.suggestions[0].exact_string, "nodejs");
        assert_eq!(results.suggestions[1].exact_string, "python");
    }

    #[test]
    fn test_installed_versions_post_process() {
        let gen = generator();
        let generator = gen
            .generators()
            .iter()
            .find(|(name, _)| name.0 == "installed_versions")
            .unwrap()
            .1;

        let output = "  18.20.0\n *20.11.1\n  22.0.0\n";
        let results = generator.on_complete(output);
        assert_eq!(results.suggestions.len(), 3);
        assert_eq!(results.suggestions[0].exact_string, "18.20.0");
        assert_eq!(results.suggestions[1].exact_string, "20.11.1");
        assert_eq!(results.suggestions[2].exact_string, "22.0.0");
    }

    #[test]
    fn test_all_versions_post_process() {
        let gen = generator();
        let generator = gen
            .generators()
            .iter()
            .find(|(name, _)| name.0 == "all_versions")
            .unwrap()
            .1;

        let output = "18.0.0\n20.0.0\n22.0.0\n";
        let results = generator.on_complete(output);
        // Reversed so latest appears first
        assert_eq!(results.suggestions.len(), 3);
        assert_eq!(results.suggestions[0].exact_string, "22.0.0");
        assert_eq!(results.suggestions[1].exact_string, "20.0.0");
        assert_eq!(results.suggestions[2].exact_string, "18.0.0");
    }

    #[test]
    fn test_plugin_from_tokens_with_trailing_whitespace() {
        // "asdf global nodejs " -> tokens = ["asdf", "global", "nodejs"], trailing = true
        let tokens = vec!["asdf", "global", "nodejs"];
        assert_eq!(plugin_from_tokens(&tokens, true), Some("nodejs"));
    }

    #[test]
    fn test_plugin_from_tokens_without_trailing_whitespace() {
        // "asdf global nodejs 20" -> tokens = ["asdf", "global", "nodejs", "20"], trailing = false
        let tokens = vec!["asdf", "global", "nodejs", "20"];
        assert_eq!(plugin_from_tokens(&tokens, false), Some("nodejs"));
    }
}
