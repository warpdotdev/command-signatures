use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use super::common;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("killall")
        .add_generator("user_name", common::users_generator())
        .add_generator(
            "process_name",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command("ps -A -o comm"),
                    CommandBuilder::single_command("sort -u"),
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|path| {
                            path.rsplit_once('/').and_then(|(_, name)| {
                                if !name.is_empty() {
                                    Some(Suggestion::with_description(name, path))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
