use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gh")
        .add_generator(
            "list_pr",
            Generator::script(CommandBuilder::single_command("gh pr list"), |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let capture = RE.captures(line);
                        if let Some(capture) = capture {
                            let captured = (
                                capture.name("id"),
                                capture.name("name"),
                                capture.name("branch"),
                            );
                            if let (Some(id), Some(name), Some(branch)) = captured {
                                return Some(Suggestion::with_description(
                                    id.as_str(),
                                    format!("#{} | {}", name.as_str(), branch.as_str()),
                                ));
                            }
                        }
                        None
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_alias",
            Generator::script(CommandBuilder::single_command("gh alias list"), |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let split: Vec<&str> = line.split(':').collect();
                        (split.len() >= 2)
                            .then(|| Suggestion::with_description(split[0].trim(), split[1].trim()))
                    })
                    .collect_unordered_results()
            }),
        )
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<id>[\d]+)\t(?P<name>.+)\t(?P<branch>.*)\t(?P<status>OPEN|DRAFT)$")
            .unwrap();
}
