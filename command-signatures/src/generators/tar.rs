use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, IconType, PriorityV1,
    Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tar").add_generator(
        "list_tar_files",
        Generator::script("ls -1 | grep '.tar'", |output| {
            output
                .trim()
                .split('\n')
                .map(|line| Suggestion {
                    exact_string: line.to_owned(),
                    description: None,
                    priority: PriorityV1::Default,
                    icon: Some(IconType::File),
                    is_hidden: false,
                    display_name: None,
                })
                .collect_unordered_results()
        }),
    )
}
