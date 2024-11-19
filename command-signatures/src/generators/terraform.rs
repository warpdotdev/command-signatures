use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("terraform")
        .add_generator(
            "workspace_list",
            Generator::script(
                CommandBuilder::single_command("terraform workspace list"),
                |output| {
                    output
                        .trim()
                        .split('\n')
                        .map(|workspace| {
                            Suggestion::with_description(
                                workspace.replace("* ", "").trim(),
                                "workspace",
                            )
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "address_list",
            Generator::script(
                CommandBuilder::single_command("terraform state list"),
                |output| {
                    if output.contains("No state file was found!") || output.contains("Error") {
                        return GeneratorResults::default();
                    }

                    output
                        .split('\n')
                        .map(|address| {
                            Suggestion::with_description(
                                address.replace("* ", "").trim(),
                                "Address",
                            )
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
