use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator(
            "units",
            Generator::script(
                CommandBuilder::single_command(
                    "systemctl list-units --no-legend --no-pager --plain --all",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut cols = line.split_whitespace();
                            let unit = cols.next()?;
                            // Skip header-like lines
                            if unit.is_empty() {
                                return None;
                            }
                            // Columns: UNIT LOAD ACTIVE SUB DESCRIPTION...
                            let _load = cols.next();
                            let active = cols.next().unwrap_or_default();
                            let sub = cols.next().unwrap_or_default();
                            let description = if !active.is_empty() && !sub.is_empty() {
                                format!("{active} ({sub})")
                            } else {
                                String::new()
                            };
                            Some(Suggestion::with_description(unit, &description))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_files",
            Generator::script(
                CommandBuilder::single_command("systemctl list-unit-files --no-legend --no-pager"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let mut cols = line.split_whitespace();
                            let unit = cols.next()?;
                            if unit.is_empty() {
                                return None;
                            }
                            let state = cols.next().unwrap_or_default();
                            Some(Suggestion::with_description(unit, state))
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
                        .filter(|line| !line.ends_with(':') && !line.is_empty())
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
                        .filter(|line| !line.ends_with(':') && !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Unit state"))
                        .collect_unordered_results()
                },
            ),
        )
}
