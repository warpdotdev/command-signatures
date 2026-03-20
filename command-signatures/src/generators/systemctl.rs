use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator(
            "units",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-units --full --no-legend --no-pager --plain --all",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.splitn(2, char::is_whitespace);
                            let unit_name = parts.next()?;
                            if unit_name.is_empty() {
                                return None;
                            }
                            // The description is the last whitespace-separated field group,
                            // but we just use the unit name with a generic description.
                            Some(Suggestion::with_description(unit_name, "Unit"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_files",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-unit-files --full --no-legend --no-pager --plain --all",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let unit_name = parts.next()?;
                            let state = parts.next().unwrap_or("unknown");
                            if unit_name.is_empty() {
                                return None;
                            }
                            Some(Suggestion::with_description(unit_name, state))
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
                        .filter(|line| !line.is_empty() && !line.ends_with(':'))
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
                        .filter(|line| !line.is_empty() && !line.ends_with(':'))
                        .map(|line| Suggestion::with_description(line.trim(), "State"))
                        .collect_unordered_results()
                },
            ),
        )
}
