use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?:[a-zA-Z0-9-]+\.)+[a-zA-Z0-9]+").unwrap();
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("ssh")
        .add_generator(
            "hosts",
            Generator::new("cat ~/.ssh/config", |output| {
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
            }),
        )
        .add_generator(
            "known_hosts",
            Generator::new("cat ~/.ssh/known_hosts", |output| {
                output
                    .lines()
                    .filter_map(|line| line.split_once(' ').map(|(first, _)| first))
                    .map(|known_host| Suggestion::with_description(known_host, "SSH Host"))
                    .collect_unordered_results()
            }),
        )
}
