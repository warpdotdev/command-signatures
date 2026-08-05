use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use super::common;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kill")
        .add_generator(
            "process",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command("ps axo pid,comm"),
                    CommandBuilder::single_command("sed 1d"),
                ),
                |output| {
                    output
                        .lines()
                        .filter_map(|line| {
                            let mut result = line.split_whitespace();

                            result
                                .next()
                                .zip(result.next())
                                .map(|(pid, path)| Suggestion::with_description(pid, path))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator("signal_name", common::signal_names_generator())
}
