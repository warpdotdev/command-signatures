use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("ssh").add_generator(
        "addresses",
        Generator::new("cat ~/.ssh/config", |output| {
            output
                .split('\n')
                .filter_map(|line| {
                    if line.trim().starts_with("Host ") && !line.contains('*') {
                        line.split_whitespace()
                            .next_back()
                            .map(|name| Suggestion::with_description(name, "ssh host"))
                    } else {
                        None
                    }
                })
                .collect_unordered_results()
        }),
    )
}
