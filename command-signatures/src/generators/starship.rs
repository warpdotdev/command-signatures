use super::output_parsers;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

fn modules_post_process(output: &str) -> GeneratorResults {
    // Starship prints "Supported modules list" and a separator before the module names.
    output
        .lines()
        .skip(2)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(|name| Suggestion::with_description(name, "Starship module"))
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("starship")
        .add_generator(
            "modules",
            Generator::script(
                CommandBuilder::single_command("starship module --list"),
                modules_post_process,
            ),
        )
        .add_generator(
            "presets",
            Generator::script(
                CommandBuilder::single_command("starship preset --list"),
                output_parsers::lines,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::modules_post_process;

    #[test]
    fn module_names_exclude_headers_and_blank_lines() {
        let output =
            "Supported modules list\n----------------------\naws\n\n  kubernetes  \nnodejs\n";
        for output in [output.to_string(), output.replace('\n', "\r\n")] {
            let suggestions = modules_post_process(&output).suggestions;
            let names: Vec<_> = suggestions
                .iter()
                .map(|suggestion| suggestion.exact_string.as_str())
                .collect();
            assert_eq!(names, ["aws", "kubernetes", "nodejs"]);
            assert!(suggestions
                .iter()
                .all(|suggestion| suggestion.description.as_deref() == Some("Starship module")));
        }
    }

    #[test]
    fn empty_or_header_only_module_lists_have_no_suggestions() {
        for output in [
            "",
            "\r\n",
            "Supported modules list\n",
            "Supported modules list\r\n----------------------\r\n\r\n",
        ] {
            assert!(modules_post_process(output).suggestions.is_empty());
        }
    }

    #[cfg(feature = "embed-signatures")]
    #[test]
    fn starship_spec_connects_named_arguments_to_its_generators() {
        use warp_completion_metadata::{ArgumentType, GeneratorName};

        let signature = crate::signature_by_name("starship").expect("starship spec should exist");
        let dynamic_data = crate::generators::dynamic_command_signature_data();
        let generators = dynamic_data
            .get("starship")
            .expect("starship generators should be registered");

        for (command, generator) in [
            ("toggle", "modules"),
            ("module", "modules"),
            ("preset", "presets"),
        ] {
            let subcommand = signature
                .subcommands()
                .iter()
                .find(|subcommand| subcommand.name == command)
                .expect("starship subcommand should exist");
            let generator_name = GeneratorName::new(generator);
            assert!(subcommand.arguments()[0]
                .argument_types
                .contains(&ArgumentType::Generator(generator_name.clone())));
            assert!(generators.generators().contains_key(&generator_name));
        }

        let toggle = signature
            .subcommands()
            .iter()
            .find(|subcommand| subcommand.name == "toggle")
            .unwrap();
        let arguments = toggle.arguments();
        assert_eq!(arguments.len(), 2);
        assert!(arguments[0].is_required());
        assert!(arguments[0].skip_generator_validation);
        assert!(!arguments[1].is_required());
        assert!(arguments.iter().all(|argument| !argument.is_variadic()));
        assert!(arguments[1]
            .argument_types
            .iter()
            .all(|kind| !matches!(kind, ArgumentType::Generator(_))));
    }
}
