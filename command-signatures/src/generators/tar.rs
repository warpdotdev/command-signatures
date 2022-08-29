use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, IconType, Priority, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("tar").add_generator(
        "list_tar_files",
        Generator::script("ls -1 | grep '.tar'", |output| {
            output
                .trim()
                .split('\n')
                .map(|line| Suggestion {
                    exact_string: line.to_owned(),
                    description: None,
                    priority: Priority::Default,
                    icon: Some(IconType::File),
                    is_hidden: false,
                })
                .collect_unordered_results()
        }),
    )
}
