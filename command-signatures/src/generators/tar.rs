use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("tar").add_generator(
        "list_tar_files",
        Generator::script("ls -1 | grep '.tar'", |output| {
            output
                .trim()
                .split('\n')
                .map(Suggestion::new)
                .collect_unordered_results()
        }),
    )
}
