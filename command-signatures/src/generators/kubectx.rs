use std::iter;

use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Importance, Order,
    Priority, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("kubectx")
        .add_generator(
            "delete_context",
            Generator::script("kubectx", |output| {
                let mut default = Suggestion::with_description(".", "Delete current context");
                default.priority = Priority::Global(Importance::More(Order(90)));

                let results = output
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(|item| {
                        let mut suggestion = Suggestion::new(item);
                        suggestion.priority = Priority::Global(Importance::More(Order(95)));
                        suggestion
                    });

                iter::once(default)
                    .chain(results)
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "kubectx_context",
            Generator::script("kubectx | grep -v $(kubectx -c)", |output| {
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
            Generator::script("kubectx -c", |output| {
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
