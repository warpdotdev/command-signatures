#[cfg(feature = "embed-signatures")]
mod assets;
mod generators;
pub mod overrides;
pub mod powershell_autogenerator;

pub use generators::dynamic_command_signature_data;

#[cfg(feature = "embed-signatures")]
use assets::Assets;
pub use warp_completion_metadata::*;

#[cfg(feature = "embed-signatures")]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    let file_path = format!("{}.json", name.as_ref());
    Assets::get(&file_path).and_then(|embedded_file| {
        let json_content = std::str::from_utf8(&embedded_file.data).ok()?;
        let fig_command: warp_completion_metadata::fig_types::Command =
            serde_json::from_str(json_content).ok()?;
        let signatures: Vec<Signature> = fig_command.into();
        debug_assert!(
            signatures.len() <= 1,
            "Tried to fetch a signature by name for a signature that has multiple names"
        );
        signatures.into_iter().next()
    })
}

/// On web, we don't embed command signatures into the binary. All requests for a command signature return
/// None. In the future, we would like to investigate lazy loading this data.
#[cfg(not(feature = "embed-signatures"))]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    None
}

#[cfg(feature = "embed-signatures")]
pub fn commands() -> Vec<Signature> {
    use itertools::Itertools;
    use rayon::prelude::*;

    Assets::iter()
        .collect_vec()
        .into_par_iter()
        .map(|path| Assets::get(&path))
        .filter_map(|embedded_file| {
            let embedded_data = embedded_file?.data;
            let json_content = std::str::from_utf8(&embedded_data).ok()?;
            let fig_command: warp_completion_metadata::fig_types::Command =
                serde_json::from_str(json_content).ok()?;
            Some(Vec::from(fig_command))
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::assets::all_signature_names;

    use super::*;

    fn get_generator_names_from_argument(arg: &Argument) -> Vec<&str> {
        let mut names = vec![];
        for arg_type in &arg.argument_types {
            if let ArgumentType::Generator(GeneratorName(name)) = arg_type {
                names.push(name.as_str());
            }
        }
        names
    }

    fn get_generator_names_from_option(opt: &Opt) -> Vec<&str> {
        opt.arguments()
            .iter()
            .flat_map(get_generator_names_from_argument)
            .collect_vec()
    }

    fn get_generator_names_from_signature(signature: &Signature) -> Vec<(&str, &str)> {
        std::iter::repeat(signature.name.as_str())
            .zip(
                // Combine generator names from arguments...
                signature
                    .arguments()
                    .iter()
                    .flat_map(get_generator_names_from_argument)
                    // generator names from options...
                    .chain(
                        signature
                            .options()
                            .iter()
                            .flat_map(get_generator_names_from_option),
                    )
                    // and generator names from subcommands.
                    .chain(
                        signature
                            .subcommands()
                            .iter()
                            .flat_map(get_generator_names_from_signature)
                            .map(|(_signature_name, generator_name)| generator_name),
                    ),
            )
            .collect_vec()
    }

    /// Verify that all generators referenced by command signatures are actually defined.
    #[test]
    fn all_referenced_generators_exist() {
        let generators = generators::dynamic_command_signature_data();
        let generator_names = generators
            .values()
            .flat_map(|dynamic_data| dynamic_data.generators().keys().map(|g| g.0.as_str()))
            .collect::<HashSet<_>>();
        assert!(
            !generator_names.is_empty(),
            "The bundled command signatures should reference at least one generator"
        );
        for signature in commands() {
            for (signature_name, generator_name) in get_generator_names_from_signature(&signature) {
                assert!(generator_names.contains(generator_name), "Did not find generator with name {generator_name} (from signature {signature_name})");
            }
        }
    }

    #[test]
    fn all_referenced_alias_generators_exist() {
        let generators = generators::dynamic_command_signature_data();
        let alias_generator_names = generators
            .values()
            .flat_map(|dynamic_data| dynamic_data.aliases().keys().map(|g| g.0.as_str()))
            .collect::<HashSet<_>>();
        assert!(
            !alias_generator_names.is_empty(),
            "The bundled command signatures should reference at least one alias generator"
        );
        for signature in commands() {
            if let Some(alias_generator_name) = signature.alias_generator {
                assert!(
                    alias_generator_names.contains(alias_generator_name.0.as_str()),
                    "Did not find generator with name {alias_generator_name} (from signature {})",
                    signature.name
                );
            }
        }
    }

    /// Verify that all command signatures are well-formed JSON and valid for our deserialization
    /// schema.
    #[test]
    fn all_command_specs_succeed_deserialization() {
        for name in all_signature_names() {
            signature_by_name(name).unwrap_or_else(|| panic!("{} failed to deserialize", name));
        }
    }

    /// Ensures no unquoted '\n' can be found.
    fn has_unsafe_newlines(str: &str) -> bool {
        let mut quote_char: Option<char> = None;
        let chars = str.chars().peekable();
        let mut is_escaped = false;

        for c in chars {
            match c {
                '\'' | '"' => {
                    if !is_escaped {
                        if quote_char.is_none() {
                            quote_char = Some(c);
                        } else if quote_char == Some(c) {
                            quote_char = None;
                        }
                    }
                }
                '\n' => {
                    if quote_char.is_none() && !is_escaped {
                        return true;
                    }
                }
                _ => {}
            }
            if c == '\\' {
                is_escaped = !is_escaped;
            } else {
                is_escaped = false;
            }
        }

        false
    }

    #[test]
    fn test_has_unsafe_newlines() {
        assert!(!has_unsafe_newlines("echo 'ahoy\nworld'"));
        assert!(has_unsafe_newlines("echo \\'bon voyage\nworld'"));
        assert!(!has_unsafe_newlines("echo \\\\'bon voyage\nworld'"));

        assert!(!has_unsafe_newlines("echo \"ciao\nworld\""));
        assert!(has_unsafe_newlines("echo \\\"danke\nworld\""));
        assert!(!has_unsafe_newlines("echo \\\\\"ello\nworld\""));

        assert!(!has_unsafe_newlines("echo \"fred's\nworld\""));
        assert!(!has_unsafe_newlines("echo 'george says \"\nworld\"'"));

        assert!(!has_unsafe_newlines("echo hello\\nworld"));
        assert!(has_unsafe_newlines("echo imagine\nworld"));
    }

    #[test]
    fn all_command_specs_have_no_newlines() {
        let generators = generators::dynamic_command_signature_data();
        let generator_names = generators.keys().collect::<HashSet<_>>();

        let token_test_cases = vec![
            "true", "test",
            "\\n",
            // Note: We don't yet check if passing in strings which include newlines are safe.
            // Many commands would blindly pass in a newline and not sanitize it, this
            // may be the intended behavior but that means we can't test for it.
            // "\n"
        ];

        for generator_name in generator_names {
            generators
                .get(generator_name)
                .unwrap()
                .generators()
                .values()
                .for_each(|generator| match &generator.process {
                    GeneratorProcess::CommandFromTokens(func) => {
                        token_test_cases.iter().for_each(|&tokens| {
                            let true_result = func(&[tokens], true);
                            assert!(
                                !has_unsafe_newlines(&true_result),
                                "[true] Tokens: `{}` - Generator `{}` has an unquoted newline in it: `{}`",
                                tokens,
                                generator_name,
                                true_result
                            );
                            let false_result = func(&[tokens], false);
                            assert!(
                                !has_unsafe_newlines(&false_result),
                                "[false] Tokens: `{}` - Generator `{}` has an unquoted newline in it: `{}`",
                                tokens,
                                generator_name,
                                false_result
                            );
                        });
                    }
                    GeneratorProcess::ShellCommand(str) => {
                        assert!(
                            !has_unsafe_newlines(str),
                            "Generator `{}` has an unquoted newline in it: `{}`",
                            generator_name,
                            str
                        );
                    }
                });
        }
    }
}
