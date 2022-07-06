use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("defaults").add_generator(
        "domain_generator",
        Generator::new("defaults domain", |output| {
            output
                .trim()
                .split(',')
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        }),
    )
}
