use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("pip").add_generator(
        "list_packages",
        Generator::new("php list", |output| {
            output
                .split('\n')
                .skip(2)
                .map(Suggestion::new)
                .collect_unordered_results()
        }),
    )
}
