use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kill")
        .add_generator(
            "process",
            Generator::script("ps axo pid,comm | sed 1d", |output| {
                output
                    .lines()
                    .filter_map(|line| {
                        let mut result = line.split_whitespace();

                        result
                            .next()
                            .zip(result.next())
                            .map(|(pid, path)| Suggestion::with_description(pid, path))
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "signal_name",
            Generator::script("env kill -l", |output| {
                RE.find_iter(output)
                    .map(|capture| Suggestion::new(capture.as_str()))
                    .collect_unordered_results()
            }),
        )
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+)").unwrap();
}
