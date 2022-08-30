use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

fn hosts(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            if line.trim().starts_with("Host ") && !line.contains('*') {
                line.split_whitespace()
                    .next_back()
                    .map(|name| Suggestion::with_description(name, "SSH Host"))
            } else {
                None
            }
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ssh")
        .add_generator("hosts", Generator::script("cat ~/.ssh/config", hosts))
        .add_generator("addresses", Generator::script("cat ~/.ssh/config", hosts))
        .add_generator(
            "known_hosts",
            Generator::script("cat ~/.ssh/known_hosts", |output| {
                output
                    .lines()
                    .filter_map(|line| line.split_once(' ').map(|(first, _)| first))
                    .map(|known_host| Suggestion::with_description(known_host, "SSH Host"))
                    .collect_unordered_results()
            }),
        )
}
