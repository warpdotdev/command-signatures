use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("killall")
        .add_generator(
            "user_name",
            Generator::new("dscl . -list /Users | grep -v '^_'", |output| {
                output
                    .trim()
                    .lines()
                    .map(Suggestion::new)
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "process_name",
            Generator::new("ps -A -o comm | sort -u", |output| {
                output
                    .trim()
                    .lines()
                    .filter_map(|path| {
                        path.rsplit_once('/').and_then(|(_, name)| {
                            if !name.is_empty() {
                                Some(Suggestion::with_description(name, path))
                            } else {
                                None
                            }
                        })
                    })
                    .collect_unordered_results()
            }),
        )
}
