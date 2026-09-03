use warp_completion_metadata::CommandSignatureGenerators;

use crate::generators::common::{dependencies_generator, get_scripts_generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("bun")
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("dependencies_generator", dependencies_generator())
        .add_generator(
            "npms_search",
            warp_completion_metadata::Generator::command_from_tokens(
                crate::generators::fig_token::npms_search,
                crate::generators::output_parsers::npms_search_results,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp_completion_metadata::{Argument, ArgumentType, DynamicCompletionData};

    #[cfg(feature = "embed-signatures")]
    fn uses_generator(argument: &Argument, generator_name: &str) -> bool {
        argument.argument_types.iter().any(|argument_type| {
            matches!(argument_type, ArgumentType::Generator(name) if name.0 == generator_name)
        })
    }

    /// `bun <script>` runs a package.json script without the `run` subcommand, so the top-level
    /// argument must offer the same script suggestions that `pnpm` and `yarn` do.
    #[cfg(feature = "embed-signatures")]
    #[test]
    fn test_bun_top_level_argument_completes_package_json_scripts() {
        let signature = crate::signature_by_name("bun").expect("bun signature should be bundled");
        let script = signature
            .arguments()
            .first()
            .expect("bun should accept a positional script argument");

        assert!(
            uses_generator(script, "get_scripts_generator"),
            "bun's top-level argument should use the get_scripts_generator, got {:?}",
            script.argument_types
        );
        // A required top-level argument would swallow the token that selects a subcommand.
        assert!(
            !script.is_required(),
            "bun's top-level script argument should be optional so named subcommands still match"
        );
        assert!(
            script.is_variadic(),
            "bun passes trailing tokens through to the script it runs"
        );
    }

    #[cfg(feature = "embed-signatures")]
    #[test]
    fn test_bun_run_argument_completes_package_json_scripts() {
        let signature = crate::signature_by_name("bun").expect("bun signature should be bundled");
        let run = signature
            .subcommands()
            .iter()
            .find(|subcommand| subcommand.name == "run")
            .expect("bun should have a run subcommand");
        let script = run
            .arguments()
            .first()
            .expect("bun run should accept a positional script argument");

        assert!(
            uses_generator(script, "get_scripts_generator"),
            "bun run's argument should use the get_scripts_generator, got {:?}",
            script.argument_types
        );
    }

    #[cfg(feature = "embed-signatures")]
    #[test]
    fn test_bun_remove_argument_completes_installed_dependencies() {
        let signature = crate::signature_by_name("bun").expect("bun signature should be bundled");

        for name in ["rm", "remove"] {
            let remove = signature
                .subcommands()
                .iter()
                .find(|subcommand| subcommand.name == name)
                .unwrap_or_else(|| panic!("bun should have a {name} subcommand"));
            let package = remove.arguments().first().unwrap_or_else(|| {
                panic!("bun {name} should accept a positional package argument")
            });

            assert!(
                uses_generator(package, "dependencies_generator"),
                "bun {name}'s argument should use the dependencies_generator, got {:?}",
                package.argument_types
            );
        }
    }

    /// The top-level script argument must not displace bun's named subcommands.
    #[cfg(feature = "embed-signatures")]
    #[test]
    fn test_bun_named_subcommands_are_still_offered() {
        let signature = crate::signature_by_name("bun").expect("bun signature should be bundled");
        let names: Vec<&str> = signature
            .subcommands()
            .iter()
            .map(|subcommand| subcommand.name.as_str())
            .collect();

        for expected in [
            "dev",
            "create",
            "run",
            "install",
            "add",
            "remove",
            "upgrade",
            "completions",
            "help",
        ] {
            assert!(
                names.contains(&expected),
                "bun should keep the {expected} subcommand, got {names:?}"
            );
        }
    }

    #[test]
    fn test_bun_registers_the_generators_its_spec_references() {
        let (command, data): (String, DynamicCompletionData) = generator().into();
        let names: Vec<&str> = data
            .generators()
            .keys()
            .map(|name| name.0.as_str())
            .collect();

        assert_eq!(command, "bun");
        for expected in ["get_scripts_generator", "dependencies_generator"] {
            assert!(
                names.contains(&expected),
                "bun should register the {expected} generator, got {names:?}"
            );
        }
    }

    #[test]
    fn test_get_scripts_generator_lists_package_json_scripts() {
        let output = r#"{
        "name": "fixture",
        "scripts": {
            "build": "bun build ./index.ts",
            "dev": "bun --watch ./index.ts",
            "test": "bun test"
        },
        "dependencies": {
            "zod": "^3.22.4"
        }
    }"#;

        let results = get_scripts_generator().on_complete(output);
        let mut suggestions: Vec<(&str, Option<&str>)> = results
            .suggestions
            .iter()
            .map(|suggestion| {
                (
                    suggestion.exact_string.as_str(),
                    suggestion.description.as_deref(),
                )
            })
            .collect();
        suggestions.sort();

        assert_eq!(
            suggestions,
            vec![
                ("build", Some("bun build ./index.ts")),
                ("dev", Some("bun --watch ./index.ts")),
                ("test", Some("bun test")),
            ]
        );
    }

    #[test]
    fn test_get_scripts_generator_ignores_a_package_json_without_scripts() {
        let output = r#"{ "name": "fixture", "dependencies": { "zod": "^3.22.4" } }"#;
        let results = get_scripts_generator().on_complete(output);

        assert!(results.suggestions.is_empty());
    }
}
