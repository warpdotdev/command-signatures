use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("kill")
        .add_generator(
            "process",
            Generator::new("ps axo pid,comm | sed 1d", |output| {
                output
                    .lines()
                    .filter_map(|line| {
                        let mut result = line.split_whitespace();

                        let pid = result.next();
                        let path = result.next();

                        match (pid, path) {
                            (Some(pid), Some(path)) => {
                                Some(Suggestion::with_description(pid, path))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "signal_name",
            Generator::new("env kill -l", |output| {
                RE.captures_iter(output)
                    .into_iter()
                    .map(|capture| Suggestion::new(&capture[1]))
                    .collect_unordered_results()
            }),
        )
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+)").unwrap();
}
