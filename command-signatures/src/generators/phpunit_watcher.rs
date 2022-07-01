use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("phpunit").add_generator(
        "tests",
        Generator::new("phpunit --list-tests", |output| {
            if output.starts_with("fatal:") {
                return GeneratorResults::empty();
            }

            output
                .split('\n')
                .filter_map(|line| {
                    if let Some(index) = line.find("::") {
                        if index > 0 && (index + 2 < line.len()) {
                            return Some(Suggestion::new(&line[index + 2..]));
                        }
                    }
                    None
                })
                .collect_from_unordered_suggestions()
        }),
    )
}
