use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("aws")
        .add_generator(
            "profiles",
            Generator::script(
                CommandBuilder::single_command(
                    "cat ~/.aws/config ~/.aws/credentials 2>/dev/null | grep '^\\[' | sed 's/\\[profile //;s/\\[//;s/\\]//' | sort -u",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "AWS profile"))
                        .collect_unordered_results()
                },
            ),
        )
}
