use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\s*\*").unwrap();
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("pyenv")
        .add_generator(
            "version_list",
            Generator::new("pyenv install -l", |output| {
                output
                    .split('\n')
                    .skip(1)
                    .filter_map(|line| (!line.is_empty()).then(|| Suggestion::new(line.trim())))
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "global_list",
            Generator::new("pyenv versions", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        (!line.is_empty()).then(|| {
                            if RE.is_match(line) {
                                Suggestion::new(line.replace('*', "").trim())
                            } else {
                                Suggestion::new(line.trim())
                            }
                        })
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
}
