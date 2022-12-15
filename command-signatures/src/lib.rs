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
