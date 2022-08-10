use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("go").add_generator(
        "tool_generator",
        Generator::script("go tool", |output| {
            output
                .split('\n')
                .map(Suggestion::new)
                .collect_unordered_results()
        }),
    )
}
