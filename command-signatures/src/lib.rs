mod generators;

pub use generators::dynamic_command_signature_data;

pub use warp_completion_metadata::*;

#[cfg(feature = "embed-signatures")]
#[derive(rust_embed::RustEmbed)]
#[folder = "json"]
struct Assets;

#[cfg(feature = "embed-signatures")]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    let file_path = format!("{}.json", name.as_ref());
    Assets::get(&file_path).and_then(|embedded_file| {
        let json_content = std::str::from_utf8(&embedded_file.data).ok()?;
        let fig_command: warp_completion_metadata::fig_types::Command =
            serde_json::from_str(json_content).ok()?;
        Some(Signature::from(fig_command))
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
            Some(Signature::from(fig_command))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

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
}
