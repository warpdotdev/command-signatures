use lazy_static::lazy_static;
use regex::Regex;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\s*\*").unwrap();
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pyenv")
        .add_generator(
            "version_list",
            Generator::script(
                CommandBuilder::single_command("pyenv install -l"),
                |output| {
                    output
                        .split('\n')
                        .skip(1)
                        .filter(|&line| !line.is_empty())
                        .map(|line| Suggestion::new(line.trim()))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "global_list",
            Generator::script(CommandBuilder::single_command("pyenv versions"), |output| {
                output
                    .split('\n')
                    .filter(|&line| !line.is_empty())
                    .map(|line| {
                        if RE.is_match(line) {
                            Suggestion::new(line.replace('*', "").trim())
                        } else {
                            Suggestion::new(line.trim())
                        }
                    })
                    .collect_unordered_results()
            }),
        )
}
