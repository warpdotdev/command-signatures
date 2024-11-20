use regex::Regex;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("firebase").add_generator(
        "project_aliases",
        Generator::script(
            CommandBuilder::single_command("firebase projects:list"),
            |output| {
                RE.captures_iter(output)
                    // First element is the table header
                    .skip(1)
                    .filter_map(|capture| {
                        capture.get(1).map(|project_name| {
                            Suggestion::with_description(
                                project_name.as_str().trim(),
                                "ProjectAlias",
                            )
                        })
                    })
                    .collect_unordered_results()
            },
        ),
    )
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)^│ (\w.*?)│").unwrap();
}
