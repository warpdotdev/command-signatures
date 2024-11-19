use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kubens")
        .add_generator(
            "kubens_context",
            Generator::script(
                CommandBuilder::single_command("kubens | grep -v $(kubens -c)"),
                |output| {
                    output
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .map(Suggestion::new)
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "context",
            Generator::script(CommandBuilder::single_command("kubens -c"), |output| {
                if output.is_empty() {
                    GeneratorResults::default()
                } else {
                    GeneratorResults {
                        suggestions: vec![Suggestion::new(output)],
                        is_ordered: false,
                    }
                }
            }),
        )
}
