use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("kubectx")
        .add_generator(
            "delete_context",
            Generator::new("kubectx", |output| {
                let default = Suggestion::with_description(".", "Delete current context");

                let mut results = output
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(Suggestion::new)
                    .collect_ordered_results();

                results.suggestions.insert(0, default);
                results
            }),
        )
        .add_generator(
            "kubectx_context",
            Generator::new("kubectx | grep -v $(kubectx -c)", |output| {
                output
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(Suggestion::new)
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "context",
            Generator::new("kubectx -c", |output| {
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
