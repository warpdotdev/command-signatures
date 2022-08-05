use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

fn tmux_post_process(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let mut result = line.split(':');

            result
                .next()
                .zip(result.next())
                .map(|(name, description)| Suggestion::with_description(name, description))
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("tmux")
        .add_generator(
            "target_session",
            Generator::new("tmux ls", tmux_post_process),
        )
        .add_generator(
            "target_client",
            Generator::new("tmux lsc", tmux_post_process),
        )
        .add_generator("src_pane", Generator::new("tmux lsp", tmux_post_process))
        .add_generator("window_name", Generator::new("tmux lsw", tmux_post_process))
        .add_generator("buffer_name", Generator::new("tmux lsb", tmux_post_process))
}
