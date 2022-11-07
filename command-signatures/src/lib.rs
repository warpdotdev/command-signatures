mod generators;

pub use generators::command_signature_generators;

pub use warp_completion_metadata::*;

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
