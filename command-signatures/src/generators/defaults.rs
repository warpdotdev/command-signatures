use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("defaults").add_generator(
        "domain_generator",
        Generator::script("defaults domain", |output| {
            output
                .trim()
                .split(',')
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        }),
    )
}
