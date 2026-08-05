use super::common;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("systemctl")
        .add_generator("units", common::systemd_units_generator())
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
