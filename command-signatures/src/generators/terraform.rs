use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("terraform")
        .add_generator(
            "workspace_list",
            Generator::new("terraform workspace list", |output| {
                output
                    .trim()
                    .split('\n')
                    .map(|workspace| {
                        Suggestion::with_description(
                            workspace.replace("* ", "").trim(),
                            "workspace",
                        )
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "address_list",
            Generator::new("terraform state list", |output| {
                if output.contains("No state file was found!") || output.contains("Error") {
                    return GeneratorResults::empty();
                }

                output
                    .split('\n')
                    .map(|address| {
                        Suggestion::with_description(address.replace("* ", "").trim(), "Address")
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
}
