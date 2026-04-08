use std::collections::HashMap;

use serde_json::Value;
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
                        .filter(|line| !line.trim().is_empty())
                        .map(|address| {
                            Suggestion::with_description(
                                address.replace("* ", "").trim(),
                                "resource",
                            )
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "output_list",
            Generator::script(
                CommandBuilder::single_command("terraform output -json"),
                |output| {
                    let parsed: Result<HashMap<String, Value>, _> = serde_json::from_str(output);

                    match parsed {
                        Ok(outputs) => outputs
                            .into_keys()
                            .map(|name| Suggestion::with_description(name, "output"))
                            .collect_unordered_results(),
                        Err(_) => GeneratorResults::default(),
                    }
                },
            ),
        )
}
