use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator(
            "units",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-units --no-legend --no-pager --plain --all -q",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            // Skip header-like or empty lines
                            if name.is_empty() || name.starts_with("UNIT") {
                                return None;
                            }
                            let description = parts.skip(3).collect::<Vec<_>>().join(" ");
                            if description.is_empty() {
                                Some(Suggestion::new(name))
                            } else {
                                Some(Suggestion::with_description(name, &description))
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_files",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-unit-files --no-legend --no-pager --plain -q",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            if name.is_empty() {
                                return None;
                            }
                            let state = parts.next().unwrap_or("");
                            if state.is_empty() {
                                Some(Suggestion::new(name))
                            } else {
                                Some(Suggestion::with_description(name, state))
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_types",
            Generator::script(
                CommandBuilder::single_command("systemctl --type=help --no-legend --no-pager"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.contains(':') && !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Unit type"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_states",
            Generator::script(
                CommandBuilder::single_command("systemctl --state=help --no-legend --no-pager"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.contains(':') && !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Unit state"))
                        .collect_unordered_results()
                },
            ),
        )
}
