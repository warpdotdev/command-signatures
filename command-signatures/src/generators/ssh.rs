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
                        l
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
                    .filter_map(|line| RE.find(line))
                    .map(|known_host| Suggestion::with_description(known_host.as_str(), "SSH Host"))
                    .collect_unordered_results()
            }),
        )
}
