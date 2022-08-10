use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("tmuxinator")
        .add_generator(
            "projects",
            Generator::script("tmuxinator list -n", |output| {
                if output.starts_with("fatal:") {
                    return GeneratorResults::default();
                }
                output
                    .lines()
                    .skip(1)
                    .map(|line| Suggestion::with_description(line, "Project"))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "session_names",
            Generator::script("tmux ls", |output| {
                if output.starts_with("fatal:") {
                    return GeneratorResults::default();
                }
                output
                    .lines()
                    .filter_map(|line| {
                        line.split_once(':').map(|(name, description)| {
                            Suggestion::with_description(name, description)
                        })
                    })
                    .collect_unordered_results()
            }),
        )
}
