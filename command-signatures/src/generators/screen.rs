use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("screen").add_generator(
        "sessions",
        Generator::script("screen -ls | sed '$d' | sed '$d' | cat", 
        |output| {
            output
                .split('\n')
                .skip(1)
                .map(str::trim)
                .filter_map(|session_line| {
                    if session_line.is_empty() {
                        return None;
                    }
                    session_line.split('\t')
                        .next()
                        .map(Suggestion::new)
                })
                .collect_unordered_results()
        }),
    )
}