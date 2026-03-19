use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator(
            "units",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-units --full --no-legend --no-pager --plain --all 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            // Skip LOAD, ACTIVE, SUB columns to get DESCRIPTION
                            let _load = parts.next();
                            let _active = parts.next();
                            let _sub = parts.next();
                            let description: String =
                                parts.collect::<Vec<&str>>().join(" ");
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
                    "systemctl list-unit-files --full --no-legend --no-pager --plain --all 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            let state = parts.next().unwrap_or_default();
                            Some(Suggestion::with_description(name, state))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_types",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl --type=help --no-legend --no-pager 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.ends_with(':') && !line.is_empty())
                        .map(|line| Suggestion::new(line.trim()))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_states",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl --state=help --no-legend --no-pager 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.ends_with(':') && !line.is_empty())
                        .map(|line| Suggestion::new(line.trim()))
                        .collect_unordered_results()
                },
            ),
        )
}
