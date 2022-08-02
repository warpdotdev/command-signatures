use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

fn list_packages_generator() -> Generator {
    Generator::new("php list", |output| {
        output
            .split('\n')
            .skip(2)
            .map(Suggestion::new)
            .collect_unordered_results()
    })
}
pub fn generator() -> CommandGenerators {
    CommandGenerators::new("pip").add_generator("list_packages", list_packages_generator())
}

pub fn pip3_generator() -> CommandGenerators {
    CommandGenerators::new("pip3").add_generator("list_packages", list_packages_generator())
}
