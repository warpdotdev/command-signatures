use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("timedatectl").add_generator(
        "timezones",
        Generator::script(
            CommandBuilder::single_command("timedatectl list-timezones"),
            |output| {
                output
                    .trim()
                    .lines()
                    .map(|line| Suggestion::with_description(line.trim(), "Time zone"))
                    .collect_unordered_results()
            },
        ),
    )
}
