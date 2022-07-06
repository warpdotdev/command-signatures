use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("conda")
        .add_generator(
            "get_installed_packages",
            Generator::new("conda list", |output| {
                output
                    .trim()
                    .split('\n')
                    .skip(2)
                    .map(Suggestion::new)
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "get_conda_environments",
            Generator::new("conva env list", |output| {
                output
                    .trim()
                    .split('\n')
                    .skip(2)
                    .filter_map(|line| line.split(' ').next().map(Suggestion::new))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "get_conda_configs",
            Generator::new("conda config --show", |output| {
                output
                    .trim()
                    .split('\n')
                    .skip(2)
                    .filter_map(|line| line.split(':').next().map(Suggestion::new))
                    .collect_unordered_results()
            }),
        )
}
