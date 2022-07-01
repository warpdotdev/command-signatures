use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

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
                    .collect::<Vec<_>>()
            }),
        )
        .add_generator(
            "address_list",
            Generator::new("terraform state list", |output| {
                if output.contains("No state file was found!") || output.contains("Error") {
                    return vec![];
                }

                output
                    .split('\n')
                    .map(|address| {
                        Suggestion::with_description(address.replace("* ", "").trim(), "Address")
                    })
                    .collect()
            }),
        )
}
