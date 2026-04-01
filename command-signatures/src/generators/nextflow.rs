use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("nextflow")
        .add_generator(
            "run_names",
            Generator::script(
                CommandBuilder::single_command("nextflow log -q 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .map(|line| Suggestion::with_description(line.trim(), "Run name"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "downloaded_projects",
            Generator::script(
                CommandBuilder::single_command("nextflow list 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .map(|line| Suggestion::with_description(line.trim(), "Downloaded project"))
                        .collect_unordered_results()
                },
            ),
        )
}
