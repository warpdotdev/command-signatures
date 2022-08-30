use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
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

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tmux")
        .add_generator(
            "target_session",
            Generator::script("tmux ls", tmux_post_process),
        )
        .add_generator(
            "target_client",
            Generator::script("tmux lsc", tmux_post_process),
        )
        .add_generator("src_pane", Generator::script("tmux lsp", tmux_post_process))
        .add_generator(
            "window_name",
            Generator::script("tmux lsw", tmux_post_process),
        )
        .add_generator(
            "buffer_name",
            Generator::script("tmux lsb", tmux_post_process),
        )
}
