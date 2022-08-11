use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

fn list_packages_post_process(output: &str) -> GeneratorResults {
    output
        .lines()
        .skip(2)
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("pip").add_generator(
        "list_packages",
        Generator::new("pip list", list_packages_post_process),
    )
}

pub fn pip3_generator() -> CommandGenerators {
    CommandGenerators::new("pip3").add_generator(
        "list_packages",
        Generator::new("pip3 list", list_packages_post_process),
    )
}
