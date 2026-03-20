use std::collections::HashSet;

use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator(
            "units",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command(
                        "{ systemctl list-units --full --no-legend --no-pager --plain --all; systemctl list-unit-files --full --no-legend --no-pager --plain --all; }",
                    ),
                    CommandBuilder::single_command("awk '!seen[$1]++ { print }'"),
                ),
                |output| {
                    let mut seen = HashSet::new();
                    output
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            if name.is_empty() || !seen.insert(name.to_string()) {
                                return None;
                            }
                            match parts.next() {
                                Some(state) => {
                                    Some(Suggestion::with_description(name, state))
                                }
                                None => Some(Suggestion::new(name)),
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "unit_types",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr(
                    "systemctl --type=help --no-legend --no-pager",
                ),
                |output| {
                    output
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
                CommandBuilder::single_command_and_ignore_stderr(
                    "systemctl --state=help --no-legend --no-pager",
                ),
                |output| {
                    output
                        .lines()
                        .filter(|line| !line.ends_with(':') && !line.is_empty())
                        .map(|line| Suggestion::new(line.trim()))
                        .collect_unordered_results()
                },
            ),
        )
}
