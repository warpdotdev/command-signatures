use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("go").add_generator(
        "tool_generator",
        Generator::script("go tool", |output| {
            output
                .split('\n')
                .map(Suggestion::new)
                .collect_unordered_results()
        }),
    )
}
