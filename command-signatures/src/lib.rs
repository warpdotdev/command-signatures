mod generators;

pub use generators::command_signature_generators;

pub use warp_completion_metadata::*;

pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    let name = name.as_ref();
    command_signatures_1::signature_by_name(name)
        .or_else(|| command_signatures_2::signature_by_name(name))
        .or_else(|| command_signatures_3::signature_by_name(name))
        .or_else(|| command_signatures_4::signature_by_name(name))
        .or_else(|| command_signatures_5::signature_by_name(name))
        .or_else(|| command_signatures_6::signature_by_name(name))
}

pub fn commands() -> Vec<Signature> {
    command_signatures_1::signatures()
        .into_iter()
        .chain(command_signatures_2::signatures().into_iter())
        .chain(command_signatures_3::signatures().into_iter())
        .chain(command_signatures_4::signatures().into_iter())
        .chain(command_signatures_5::signatures().into_iter())
        .chain(command_signatures_6::signatures().into_iter())
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
            match arg_type {
                ArgumentType::Generator(GeneratorName(name)) => names.push(name.as_str()),
                _ => {}
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
        let generators = generators::command_signature_generators();
        let generator_names = generators
            .values()
            .flat_map(|(generators, _, _)| generators.keys().map(|g| g.0.as_str()))
            .collect::<HashSet<_>>();
        for signature in commands() {
            for (signature_name, generator_name) in get_generator_names_from_signature(&signature) {
                assert!(generator_names.contains(generator_name), "Did not find generator with name {generator_name} (from signature {signature_name})");
            }
        }
    }
}
