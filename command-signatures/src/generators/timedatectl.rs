use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("timedatectl").add_generator(
        "timezone_generator",
        Generator::script(
            CommandBuilder::single_command("timedatectl list-timezones"),
            |output| {
                output
                    .trim()
                    .lines()
                    .map(|line| Suggestion::new(line.trim()))
                    .collect_unordered_results()
            },
        ),
    )
}
