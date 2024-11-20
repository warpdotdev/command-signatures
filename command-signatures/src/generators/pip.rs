use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

fn list_packages_post_process(output: &str) -> GeneratorResults {
    output
        .lines()
        .skip(2)
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pip").add_generator(
        "list_packages",
        Generator::script(
            CommandBuilder::single_command("pip list"),
            list_packages_post_process,
        ),
    )
}

pub fn pip3_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pip3").add_generator(
        "list_packages",
        Generator::script(
            CommandBuilder::single_command("pip3 list"),
            list_packages_post_process,
        ),
    )
}
